//! A dependency-free safetensors reader — how real weights get into a
//! [`crate::runtime::Session`].
//!
//! The format is deliberately simple and so is this: an 8-byte little-endian
//! header length, a JSON header mapping tensor names to
//! `{dtype, shape, data_offsets}`, then the raw bytes. The loader decodes
//! every tensor to `f64` host-side — `BF16` and `F16` widen exactly (bf16 is
//! a truncated f32), so a bf16 checkpoint loses nothing on the way in. The
//! *storage* dtype is still visible to the planner through
//! [`crate::ir::input_dt`]; in-kernel byte storage is the part that remains
//! open (see todo.md).
//!
//! The tiny JSON parser is public because a tokenizer's `vocab.json` needs
//! exactly the same thing and nothing more.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

// ── a minimal JSON value ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(kvs) => kvs.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_num(&self) -> Option<f64> {
        match self {
            Json::Num(n) => Some(*n),
            _ => None,
        }
    }
    pub fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }
}

/// Parse a JSON document. Strict enough for safetensors headers and
/// tokenizer vocabularies; not a general-purpose validator.
pub fn parse_json(src: &str) -> Result<Json, String> {
    let b = src.as_bytes();
    let mut i = 0usize;
    let v = parse_value(b, &mut i)?;
    skip_ws(b, &mut i);
    if i != b.len() {
        return Err(format!("trailing bytes at {i}"));
    }
    Ok(v)
}

fn skip_ws(b: &[u8], i: &mut usize) {
    while *i < b.len() && matches!(b[*i], b' ' | b'\t' | b'\n' | b'\r') {
        *i += 1;
    }
}

fn parse_value(b: &[u8], i: &mut usize) -> Result<Json, String> {
    skip_ws(b, i);
    match b.get(*i) {
        Some(b'{') => {
            *i += 1;
            let mut kvs = Vec::new();
            skip_ws(b, i);
            if b.get(*i) == Some(&b'}') {
                *i += 1;
                return Ok(Json::Obj(kvs));
            }
            loop {
                skip_ws(b, i);
                let k = parse_string(b, i)?;
                skip_ws(b, i);
                if b.get(*i) != Some(&b':') {
                    return Err(format!("expected ':' at {i}"));
                }
                *i += 1;
                let v = parse_value(b, i)?;
                kvs.push((k, v));
                skip_ws(b, i);
                match b.get(*i) {
                    Some(b',') => *i += 1,
                    Some(b'}') => {
                        *i += 1;
                        return Ok(Json::Obj(kvs));
                    }
                    _ => return Err(format!("expected ',' or '}}' at {i}")),
                }
            }
        }
        Some(b'[') => {
            *i += 1;
            let mut items = Vec::new();
            skip_ws(b, i);
            if b.get(*i) == Some(&b']') {
                *i += 1;
                return Ok(Json::Arr(items));
            }
            loop {
                items.push(parse_value(b, i)?);
                skip_ws(b, i);
                match b.get(*i) {
                    Some(b',') => *i += 1,
                    Some(b']') => {
                        *i += 1;
                        return Ok(Json::Arr(items));
                    }
                    _ => return Err(format!("expected ',' or ']' at {i}")),
                }
            }
        }
        Some(b'"') => Ok(Json::Str(parse_string(b, i)?)),
        Some(b't') if b[*i..].starts_with(b"true") => {
            *i += 4;
            Ok(Json::Bool(true))
        }
        Some(b'f') if b[*i..].starts_with(b"false") => {
            *i += 5;
            Ok(Json::Bool(false))
        }
        Some(b'n') if b[*i..].starts_with(b"null") => {
            *i += 4;
            Ok(Json::Null)
        }
        Some(_) => {
            let start = *i;
            while *i < b.len() && matches!(b[*i], b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') {
                *i += 1;
            }
            std::str::from_utf8(&b[start..*i])
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .map(Json::Num)
                .ok_or_else(|| format!("bad number at {start}"))
        }
        None => Err("unexpected end of input".into()),
    }
}

fn parse_string(b: &[u8], i: &mut usize) -> Result<String, String> {
    if b.get(*i) != Some(&b'"') {
        return Err(format!("expected '\"' at {i}"));
    }
    *i += 1;
    let mut out = String::new();
    while let Some(&c) = b.get(*i) {
        *i += 1;
        match c {
            b'"' => return Ok(out),
            b'\\' => {
                let e = *b.get(*i).ok_or("truncated escape")?;
                *i += 1;
                match e {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'b' => out.push('\u{8}'),
                    b'f' => out.push('\u{c}'),
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'u' => {
                        let hex = std::str::from_utf8(b.get(*i..*i + 4).ok_or("truncated \\u")?)
                            .map_err(|_| "bad \\u")?;
                        let mut cp =
                            u32::from_str_radix(hex, 16).map_err(|_| "bad \\u hex")? as u32;
                        *i += 4;
                        // surrogate pair
                        if (0xD800..0xDC00).contains(&cp)
                            && b.get(*i) == Some(&b'\\')
                            && b.get(*i + 1) == Some(&b'u')
                        {
                            let hex2 =
                                std::str::from_utf8(b.get(*i + 2..*i + 6).ok_or("truncated")?)
                                    .map_err(|_| "bad \\u")?;
                            let lo = u32::from_str_radix(hex2, 16).map_err(|_| "bad \\u hex")?;
                            if (0xDC00..0xE000).contains(&lo) {
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                                *i += 6;
                            }
                        }
                        out.push(char::from_u32(cp).unwrap_or('\u{FFFD}'));
                    }
                    other => return Err(format!("bad escape \\{}", other as char)),
                }
            }
            _ => {
                // copy the raw UTF-8 byte run
                let start = *i - 1;
                while *i < b.len() && b[*i] != b'"' && b[*i] != b'\\' {
                    *i += 1;
                }
                out.push_str(
                    std::str::from_utf8(&b[start..*i]).map_err(|_| "invalid utf-8 in string")?,
                );
            }
        }
    }
    Err("unterminated string".into())
}

// ── the safetensors file ─────────────────────────────────────────────────────

/// One loaded tensor: its shape and its values, widened to f64.
#[derive(Debug, Clone)]
pub struct RawTensor {
    pub shape: Vec<usize>,
    pub data: Vec<f64>,
    /// The dtype the file stored — what `input_dt` should declare so the
    /// planner prices the true bandwidth.
    pub dtype: &'static str,
}

/// An opened safetensors file with the header parsed and the payload held —
/// tensors decode (or hand out raw bytes) on demand, so a multi-gigabyte
/// quantized checkpoint never widens wholesale. `weight_packed` int32
/// tensors pass through as raw bytes for typed GPU buffers.
pub struct StFile {
    bytes: Vec<u8>,
    data_off: usize,
    entries: HashMap<String, (String, Vec<usize>, usize, usize)>,
}

impl StFile {
    pub fn open(path: &Path) -> Result<StFile, String> {
        let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        if bytes.len() < 8 {
            return Err("file too short for a safetensors header".into());
        }
        let hlen = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
        let header = std::str::from_utf8(
            bytes
                .get(8..8 + hlen)
                .ok_or("header length exceeds the file")?,
        )
        .map_err(|e| format!("header is not UTF-8: {e}"))?;
        let Json::Obj(kvs) = parse_json(header)? else {
            return Err("header is not a JSON object".into());
        };
        let mut entries = HashMap::new();
        for (name, meta) in kvs {
            if name == "__metadata__" {
                continue;
            }
            let dtype = meta
                .get("dtype")
                .and_then(Json::as_str)
                .ok_or_else(|| format!("{name}: missing dtype"))?
                .to_string();
            let shape: Vec<usize> = meta
                .get("shape")
                .and_then(Json::as_arr)
                .ok_or_else(|| format!("{name}: missing shape"))?
                .iter()
                .map(|v| v.as_num().unwrap_or(0.0) as usize)
                .collect();
            let offs = meta
                .get("data_offsets")
                .and_then(Json::as_arr)
                .ok_or_else(|| format!("{name}: missing data_offsets"))?;
            let (a, b) = (
                offs[0].as_num().unwrap() as usize,
                offs[1].as_num().unwrap() as usize,
            );
            entries.insert(name, (dtype, shape, a, b));
        }
        Ok(StFile {
            bytes,
            data_off: 8 + hlen,
            entries,
        })
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.entries.keys().map(|s| s.as_str())
    }

    pub fn has(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }

    /// (file dtype, shape) of a tensor.
    pub fn meta(&self, name: &str) -> (&str, &[usize]) {
        let (d, s, _, _) = self
            .entries
            .get(name)
            .unwrap_or_else(|| panic!("no tensor `{name}` in file"));
        (d, s)
    }

    /// The tensor's raw little-endian bytes, exactly as stored.
    pub fn raw(&self, name: &str) -> &[u8] {
        let (_, _, a, b) = self
            .entries
            .get(name)
            .unwrap_or_else(|| panic!("no tensor `{name}` in file"));
        &self.bytes[self.data_off + a..self.data_off + b]
    }

    /// Decode to f32 (BF16/F16/F32 sources).
    pub fn f32(&self, name: &str) -> Vec<f32> {
        let (dtype, _) = self.meta(name);
        let raw = self.raw(name);
        match dtype {
            "F32" => raw
                .chunks_exact(4)
                .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
                .collect(),
            "BF16" => raw
                .chunks_exact(2)
                .map(|c| {
                    f32::from_bits((u16::from_le_bytes(c.try_into().unwrap()) as u32) << 16)
                })
                .collect(),
            "F16" => raw
                .chunks_exact(2)
                .map(|c| f16_to_f32(u16::from_le_bytes(c.try_into().unwrap())))
                .collect(),
            other => panic!("{name}: cannot decode {other} to f32"),
        }
    }

    /// Decode to f64 through the same paths `load` uses.
    pub fn f64(&self, name: &str) -> Vec<f64> {
        let (dtype, _) = self.meta(name);
        decode(dtype, self.raw(name))
            .unwrap_or_else(|e| panic!("{name}: {e}"))
            .0
    }
}

/// Load every tensor in a `.safetensors` file, decoding to f64.
pub fn load(path: &Path) -> Result<HashMap<String, RawTensor>, String> {
    let f = StFile::open(path)?;
    let mut out = HashMap::new();
    for (name, (dtype, shape, a, b)) in &f.entries {
        let raw = &f.bytes[f.data_off + a..f.data_off + b];
        let (decoded, tag) = decode(dtype, raw).map_err(|e| format!("{name}: {e}"))?;
        let n: usize = shape.iter().product::<usize>().max(1);
        if decoded.len() != n {
            return Err(format!(
                "{name}: {} elements decoded, shape says {n}",
                decoded.len()
            ));
        }
        out.insert(
            name.clone(),
            RawTensor {
                shape: shape.clone(),
                data: decoded,
                dtype: tag,
            },
        );
    }
    Ok(out)
}

fn decode(dtype: &str, raw: &[u8]) -> Result<(Vec<f64>, &'static str), String> {
    let chunks = |w: usize| -> Result<std::slice::ChunksExact<'_, u8>, String> {
        if raw.len() % w != 0 {
            return Err(format!("byte length {} not a multiple of {w}", raw.len()));
        }
        Ok(raw.chunks_exact(w))
    };
    match dtype {
        "F64" => Ok((
            chunks(8)?
                .map(|c| f64::from_le_bytes(c.try_into().unwrap()))
                .collect(),
            "F64",
        )),
        "F32" => Ok((
            chunks(4)?
                .map(|c| f32::from_le_bytes(c.try_into().unwrap()) as f64)
                .collect(),
            "F32",
        )),
        // bf16 is the top 16 bits of an f32 — widening is exact.
        "BF16" => Ok((
            chunks(2)?
                .map(|c| {
                    let bits = u16::from_le_bytes(c.try_into().unwrap());
                    f32::from_bits((bits as u32) << 16) as f64
                })
                .collect(),
            "BF16",
        )),
        "F16" => Ok((
            chunks(2)?
                .map(|c| f16_to_f32(u16::from_le_bytes(c.try_into().unwrap())) as f64)
                .collect(),
            "F16",
        )),
        other => Err(format!("dtype {other} not supported by this loader")),
    }
}

fn f16_to_f32(h: u16) -> f32 {
    let sign = ((h >> 15) as u32) << 31;
    let exp = ((h >> 10) & 0x1F) as u32;
    let man = (h & 0x3FF) as u32;
    let bits = match (exp, man) {
        (0, 0) => sign,
        (0, m) => {
            // subnormal: normalize
            let shift = m.leading_zeros() - 21;
            let m2 = (m << (shift + 1)) & 0x3FF;
            sign | ((127 - 15 - shift) << 23) | (m2 << 13)
        }
        (0x1F, 0) => sign | 0x7F80_0000,
        (0x1F, m) => sign | 0x7F80_0000 | (m << 13),
        (e, m) => sign | ((e + 127 - 15) << 23) | (m << 13),
    };
    f32::from_bits(bits)
}

/// Round-trip a value through bf16 (round-to-nearest-even) — how a bf16
/// checkpoint of these weights would read back.
pub fn bf16_roundtrip(v: f64) -> f64 {
    let bits = (v as f32).to_bits();
    let rounded = bits.wrapping_add(0x7FFF + ((bits >> 16) & 1));
    f32::from_bits(rounded & 0xFFFF_0000) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_parses_headers_and_escapes() {
        let j = parse_json(r#"{"a":{"dtype":"F32","shape":[2,3],"data_offsets":[0,24]},"s":"Ġx\n"}"#)
            .unwrap();
        assert_eq!(
            j.get("a").unwrap().get("dtype").unwrap().as_str(),
            Some("F32")
        );
        assert_eq!(j.get("s").unwrap().as_str(), Some("\u{120}x\n"));
    }

    #[test]
    fn bf16_and_f16_widen_correctly() {
        // bf16(1.5) = 0x3FC0
        let (v, _) = decode("BF16", &0x3FC0u16.to_le_bytes()).unwrap();
        assert_eq!(v[0], 1.5);
        // f16(1.5) = 0x3E00, f16(-2.0) = 0xC000
        let (v, _) = decode("F16", &0x3E00u16.to_le_bytes()).unwrap();
        assert_eq!(v[0], 1.5);
        let (v, _) = decode("F16", &0xC000u16.to_le_bytes()).unwrap();
        assert_eq!(v[0], -2.0);
    }

    #[test]
    fn bf16_roundtrip_rounds_to_nearest_even() {
        assert_eq!(bf16_roundtrip(1.0), 1.0);
        let x = 1.0 + 1.0 / 512.0; // one bf16 ulp below 1.00390625 rounds
        let r = bf16_roundtrip(x);
        assert!((r - x).abs() < 1.0 / 128.0);
    }
}
