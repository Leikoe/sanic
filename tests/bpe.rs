//! The GPT-2 byte-level BPE encoder against ground truth: every expected id
//! sequence below was produced by the HuggingFace `tokenizers` ByteLevel BPE
//! on THIS repo's `weights/vocab.json` + `weights/merges.txt`. Exactness is
//! the contract — a wrong pre-tokenizer split changes ids silently.
//!
//! Skips (loudly) when the weights directory isn't populated, like the GPU
//! tests skip without a Metal device.

use sanic::bpe::Bpe;

fn load() -> Option<Bpe> {
    let (v, m) = ("weights/vocab.json", "weights/merges.txt");
    if !std::path::Path::new(v).exists() || !std::path::Path::new(m).exists() {
        eprintln!("skipping: weights/vocab.json + merges.txt not present");
        return None;
    }
    Some(Bpe::from_gpt2(v, m).expect("load BPE"))
}

#[test]
fn encodes_the_pinned_gpt2_prompt() {
    let Some(bpe) = load() else { return };
    assert_eq!(
        bpe.encode("Hello, I'm a language model,"),
        vec![15496, 11, 314, 1101, 257, 3303, 2746, 11]
    );
}

#[test]
fn matches_huggingface_on_a_gauntlet() {
    let Some(bpe) = load() else { return };
    let cases: Vec<(&str, Vec<u32>)> = vec![
        (
            "The quick brown fox jumps over the lazy dog.",
            vec![464, 2068, 7586, 21831, 18045, 625, 262, 16931, 3290, 13],
        ),
        (
            "  leading spaces and   runs\tand\nnewlines  ",
            vec![
                220, 3756, 9029, 290, 220, 220, 4539, 197, 392, 198, 3605, 6615, 220, 220,
            ],
        ),
        (
            "unicode: héllo wörld — em—dash, 中文字, emoji 🎉!",
            vec![
                46903, 1098, 25, 289, 2634, 18798, 266, 30570, 335, 851, 795, 960, 42460, 11, 220,
                40792, 23877, 229, 27764, 245, 11, 44805, 12520, 236, 231, 0,
            ],
        ),
        (
            "numbers 123 456789 mix3d, don't we'll they've I'd it's",
            vec![
                77, 17024, 17031, 4153, 3134, 4531, 5022, 18, 67, 11, 836, 470, 356, 1183, 484,
                1053, 314, 1549, 340, 338,
            ],
        ),
        (
            "CamelCase snake_case kebab-case UPPER 'quotes'",
            vec![
                34, 17983, 20448, 17522, 62, 7442, 885, 65, 397, 12, 7442, 471, 10246, 1137, 705,
                421, 6421, 6,
            ],
        ),
        ("trailing spaces   ", vec![9535, 4386, 9029, 220, 220, 220]),
        ("a", vec![64]),
        (" ", vec![220]),
        ("", vec![]),
    ];
    for (text, want) in cases {
        assert_eq!(bpe.encode(text), want, "encode({text:?})");
    }
}
