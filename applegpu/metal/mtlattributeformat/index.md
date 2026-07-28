# MTLAttributeFormat

*Enumeration · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlattributeformat>

The data format options for acceleration structures.

## Declaration

```swift
enum MTLAttributeFormat
```

## Overview

All formats use little-endian byte order, which stores the least significant byte first. For GPU compute functions that manipulate data that other parts of your app consume, check that the data it exposes to the GPU matches the byte and bit alignments of the source format.

In a GPU compute function’s attributes, you can use a type that’s different from the original source data if it has the same number of bits. For example, a GPU function can interpret a 128-bit little-endian integer as a four-component vector of unsigned 32-bit integers ([MTLAttributeFormat.uint4](https://developer.apple.com/documentation/metal/mtlattributeformat/uint4)).

> **Tip:**
>  Avoid visual corruption when manipulating pixel data in a GPU compute function for a subsequent stage by using an exact match for the underlying pixel data.

### Normalized integer formats

Normalized signed integer formats have `Normalized` in the name and signed types like [MTLAttributeFormat.char](https://developer.apple.com/documentation/metal/mtlattributeformat/char) or [MTLAttributeFormat.short](https://developer.apple.com/documentation/metal/mtlattributeformat/short). For these formats, values in the range `[-1.0, 1.0]` map to `[MIN_INT, MAX_INT]`, where `MIN_INT` is the most negative integer and `MAX_INT` is the most positive integer for the number of bits in the storage size. Positive values and zero distribute uniformly in the range `[0.0, 1.0]`, and negative integer values greater than `(MIN_INT + 1)` distribute uniformly in the range `(-1.0, 0.0)`.

> **Important:**
>  For normalized signed integer formats, the values `MIN_INT` and `(MIN_INT + 1)` both map to `-1.0`.

Normalized unsigned integer formats have `Normalized` in the name and unsigned types like [MTLAttributeFormat.uchar](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar) or [MTLAttributeFormat.ushort](https://developer.apple.com/documentation/metal/mtlattributeformat/ushort). For these formats, values in the range `[0.0, 1.0]` map to `[0, MAX_UINT]`, where `MAX_UINT` is the largest unsigned integer for the number of bits in the storage size.

## Topics

### 32-bit floating-point formats
- [MTLAttributeFormat.float](https://developer.apple.com/documentation/metal/mtlattributeformat/float) — A 32-bit floating-point value.
- [MTLAttributeFormat.float2](https://developer.apple.com/documentation/metal/mtlattributeformat/float2) — A two-component vector with 32-bit floating-point values.
- [MTLAttributeFormat.float3](https://developer.apple.com/documentation/metal/mtlattributeformat/float3) — A three-component vector with 32-bit floating-point values.
- [MTLAttributeFormat.float4](https://developer.apple.com/documentation/metal/mtlattributeformat/float4) — A four-component vector with 32-bit floating-point values.
- [MTLAttributeFormat.floatRG11B10](https://developer.apple.com/documentation/metal/mtlattributeformat/floatrg11b10) — One packed 32-bit value representing pixel data containing 11-bit float red and green channels, and a 10-bit float blue channel.
- [MTLAttributeFormat.floatRGB9E5](https://developer.apple.com/documentation/metal/mtlattributeformat/floatrgb9e5) — One packed 32-bit value representing pixel data containing 9-bit float red, green, and blue channels, and a 5-bit float shared exponent channel.

### 32-bit integer formats
- [MTLAttributeFormat.int](https://developer.apple.com/documentation/metal/mtlattributeformat/int) — A 32-bit, signed integer value.
- [MTLAttributeFormat.int2](https://developer.apple.com/documentation/metal/mtlattributeformat/int2) — A two-component vector with 32-bit, signed integer values.
- [MTLAttributeFormat.int3](https://developer.apple.com/documentation/metal/mtlattributeformat/int3) — A three-component vector with 32-bit, signed integer values.
- [MTLAttributeFormat.int4](https://developer.apple.com/documentation/metal/mtlattributeformat/int4) — A four-component vector with 32-bit, signed integer values.
- [MTLAttributeFormat.uint](https://developer.apple.com/documentation/metal/mtlattributeformat/uint) — A 32-bit, unsigned integer value.
- [MTLAttributeFormat.uint2](https://developer.apple.com/documentation/metal/mtlattributeformat/uint2) — A two-component vector with 32-bit, unsigned integer values.
- [MTLAttributeFormat.uint3](https://developer.apple.com/documentation/metal/mtlattributeformat/uint3) — A three-component vector with 32-bit, unsigned integer values.
- [MTLAttributeFormat.uint4](https://developer.apple.com/documentation/metal/mtlattributeformat/uint4) — A four-component vector with 32-bit, unsigned integer values.

### 32-bit normalized integer formats
- [MTLAttributeFormat.int1010102Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/int1010102normalized) — One packed 32-bit value with four normalized signed two’s complement integer values, arranged as 10 bits, 10 bits, 10 bits, and 2 bits.
- [MTLAttributeFormat.uint1010102Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/uint1010102normalized) — One packed 32-bit value with four normalized unsigned integer values, arranged as 10 bits, 10 bits, 10 bits, and 2 bits.
- [MTLAttributeFormat.uchar4Normalized_bgra](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar4normalized_bgra) — Four unsigned normalized 8-bit values, arranged as blue, green, red, and alpha components.

### 16-bit floating-point formats
- [MTLAttributeFormat.half](https://developer.apple.com/documentation/metal/mtlattributeformat/half) — A 16-bit floating-point value.
- [MTLAttributeFormat.half2](https://developer.apple.com/documentation/metal/mtlattributeformat/half2) — A two-component vector with 16-bit floating-point values.
- [MTLAttributeFormat.half3](https://developer.apple.com/documentation/metal/mtlattributeformat/half3) — A three-component vector with 16-bit floating-point values.
- [MTLAttributeFormat.half4](https://developer.apple.com/documentation/metal/mtlattributeformat/half4) — A four-component vector with 16-bit floating-point values.

### 16-bit integer formats
- [MTLAttributeFormat.short](https://developer.apple.com/documentation/metal/mtlattributeformat/short) — A 16-bit, signed integer value.
- [MTLAttributeFormat.short2](https://developer.apple.com/documentation/metal/mtlattributeformat/short2) — A two-component vector with 16-bit, signed integer values.
- [MTLAttributeFormat.short3](https://developer.apple.com/documentation/metal/mtlattributeformat/short3) — A three-component vector with 16-bit, signed integer values.
- [MTLAttributeFormat.short4](https://developer.apple.com/documentation/metal/mtlattributeformat/short4) — A four-component vector with 16-bit, signed integer values.
- [MTLAttributeFormat.ushort](https://developer.apple.com/documentation/metal/mtlattributeformat/ushort) — A 16-bit, unsigned integer value.
- [MTLAttributeFormat.ushort2](https://developer.apple.com/documentation/metal/mtlattributeformat/ushort2) — A two-component vector with 16-bit, unsigned integer values.
- [MTLAttributeFormat.ushort3](https://developer.apple.com/documentation/metal/mtlattributeformat/ushort3) — A three-component vector with 16-bit, unsigned integer values.
- [MTLAttributeFormat.ushort4](https://developer.apple.com/documentation/metal/mtlattributeformat/ushort4) — A four-component vector with 16-bit, unsigned integer values.

### 16-bit normalized integer formats
- [MTLAttributeFormat.shortNormalized](https://developer.apple.com/documentation/metal/mtlattributeformat/shortnormalized) — A 16-bit, normalized, signed integer value.
- [MTLAttributeFormat.short2Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/short2normalized) — A two-component vector with 16-bit, normalized, signed integer values.
- [MTLAttributeFormat.short3Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/short3normalized) — A three-component vector with 16-bit, normalized, signed integer values.
- [MTLAttributeFormat.short4Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/short4normalized) — A four-component vector with 16-bit, normalized, signed integer values.
- [MTLAttributeFormat.ushortNormalized](https://developer.apple.com/documentation/metal/mtlattributeformat/ushortnormalized) — A 16-bit, normalized, unsigned integer value.
- [MTLAttributeFormat.ushort2Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/ushort2normalized) — Two unsigned normalized 16-bit values
- [MTLAttributeFormat.ushort3Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/ushort3normalized) — A three-component vector with 16-bit, normalized, unsigned integer values.
- [MTLAttributeFormat.ushort4Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/ushort4normalized) — A four-component vector with 16-bit, normalized, unsigned integer values.

### 8-bit integer formats
- [MTLAttributeFormat.char](https://developer.apple.com/documentation/metal/mtlattributeformat/char) — An 8-bit, signed integer value.
- [MTLAttributeFormat.char2](https://developer.apple.com/documentation/metal/mtlattributeformat/char2) — A two-component vector with 8-bit, signed integer values.
- [MTLAttributeFormat.char3](https://developer.apple.com/documentation/metal/mtlattributeformat/char3) — A three-component vector with 8-bit, signed integer values.
- [MTLAttributeFormat.char4](https://developer.apple.com/documentation/metal/mtlattributeformat/char4) — A four-component vector with 8-bit, signed integer values.
- [MTLAttributeFormat.uchar](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar) — An 8-bit, unsigned integer value.
- [MTLAttributeFormat.uchar2](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar2) — A two-component vector with 8-bit, unsigned integer values.
- [MTLAttributeFormat.uchar3](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar3) — A three-component vector with 8-bit, unsigned integer values.
- [MTLAttributeFormat.uchar4](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar4) — A four-component vector with 8-bit, unsigned integer values.

### 8-bit normalized integer formats
- [MTLAttributeFormat.charNormalized](https://developer.apple.com/documentation/metal/mtlattributeformat/charnormalized) — An 8-bit, normalized, signed integer value.
- [MTLAttributeFormat.char2Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/char2normalized) — A two-component vector with 8-bit, normalized, signed integer values.
- [MTLAttributeFormat.char3Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/char3normalized) — A three-component vector with 8-bit, normalized, signed integer values.
- [MTLAttributeFormat.char4Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/char4normalized) — A four-component vector with 8-bit, normalized, signed integer values.
- [MTLAttributeFormat.ucharNormalized](https://developer.apple.com/documentation/metal/mtlattributeformat/ucharnormalized) — An 8-bit, normalized, unsigned integer value.
- [MTLAttributeFormat.uchar2Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar2normalized) — A two-component vector with 8-bit, normalized, unsigned integer values.
- [MTLAttributeFormat.uchar3Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar3normalized) — A three-component vector with 8-bit, normalized, unsigned integer values.
- [MTLAttributeFormat.uchar4Normalized](https://developer.apple.com/documentation/metal/mtlattributeformat/uchar4normalized) — A four-component vector with 8-bit, normalized, unsigned integer values.

### Sentinel values
- [MTLAttributeFormat.invalid](https://developer.apple.com/documentation/metal/mtlattributeformat/invalid) — A sentinel value that represents an invalid attribute format.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlattributeformat/init(rawvalue:))

## See also

### Defining attribute location
- [bufferIndex](https://developer.apple.com/documentation/metal/mtlattributedescriptor/bufferindex) — The index in the buffer argument table for the buffer that contains the data for this attribute.
- [offset](https://developer.apple.com/documentation/metal/mtlattributedescriptor/offset) — The offset, in bytes, from the start of the buffer that contains the attribute data to the start of the data itself.
- [format](https://developer.apple.com/documentation/metal/mtlattributedescriptor/format) — The format of the attribute’s data.
