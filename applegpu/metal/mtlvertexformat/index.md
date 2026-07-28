# MTLVertexFormat

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexformat>

The vertex data format options for render pipelines.

## Declaration

```swift
enum MTLVertexFormat
```

## Overview

Set the [format](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/format) property of [MTLVertexAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor) to one of these format values. The format configures how Metal interprets the vertex data in memory for the corresponding argument in your vertex shader. Choose a format that matches the type and component count the shader expects.

### Normalized integer formats

Normalized signed integer formats have `Normalized` in the name and signed types like [MTLVertexFormat.char](https://developer.apple.com/documentation/metal/mtlvertexformat/char) or [MTLVertexFormat.short](https://developer.apple.com/documentation/metal/mtlvertexformat/short). For these formats, values in the range `[-1.0, 1.0]` map to `[MIN_INT, MAX_INT]`, where `MIN_INT` is the most negative integer and `MAX_INT` is the most positive integer for the number of bits in the storage size. Positive values and zero distribute uniformly in the range `[0.0, 1.0]`, and negative integer values greater than `(MIN_INT + 1)` distribute uniformly in the range `(-1.0, 0.0)`.

> **Important:**
>  For normalized signed integer formats, the values `MIN_INT` and `(MIN_INT + 1)` both map to `-1.0`.

Normalized unsigned integer formats have `Normalized` in the name and unsigned types like [MTLVertexFormat.uchar](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar) or [MTLVertexFormat.ushort](https://developer.apple.com/documentation/metal/mtlvertexformat/ushort). For these formats, values in the range `[0.0, 1.0]` map to `[0, MAX_UINT]`, where `MAX_UINT` is the largest unsigned integer for the number of bits in the storage size.

Metal stores data in little-endian byte order, with the least-significant byte at the lowest memory address. Formats with multibyte components also store each component in little-endian byte order.

## Topics

### 32-bit floating-point formats
- [MTLVertexFormat.float](https://developer.apple.com/documentation/metal/mtlvertexformat/float) — A 32-bit floating-point value.
- [MTLVertexFormat.float2](https://developer.apple.com/documentation/metal/mtlvertexformat/float2) — A two-component vector with 32-bit floating-point values.
- [MTLVertexFormat.float3](https://developer.apple.com/documentation/metal/mtlvertexformat/float3) — A three-component vector with 32-bit floating-point values.
- [MTLVertexFormat.float4](https://developer.apple.com/documentation/metal/mtlvertexformat/float4) — A four-component vector with 32-bit floating-point values.
- [MTLVertexFormat.floatRG11B10](https://developer.apple.com/documentation/metal/mtlvertexformat/floatrg11b10) — A three-component vector with 11-bit floating-point values for red and green, and a 10-bit value for blue.
- [MTLVertexFormat.floatRGB9E5](https://developer.apple.com/documentation/metal/mtlvertexformat/floatrgb9e5) — A three-component vector with 9-bit floating-point values for red, green, and blue, and a 5-bit shared exponent.

### 32-bit integer formats
- [MTLVertexFormat.int](https://developer.apple.com/documentation/metal/mtlvertexformat/int) — A 32-bit, signed integer value.
- [MTLVertexFormat.int2](https://developer.apple.com/documentation/metal/mtlvertexformat/int2) — A two-component vector with 32-bit, signed integer values.
- [MTLVertexFormat.int3](https://developer.apple.com/documentation/metal/mtlvertexformat/int3) — A three-component vector with 32-bit, signed integer values.
- [MTLVertexFormat.int4](https://developer.apple.com/documentation/metal/mtlvertexformat/int4) — A four-component vector with 32-bit, signed integer values.
- [MTLVertexFormat.uint](https://developer.apple.com/documentation/metal/mtlvertexformat/uint) — A 32-bit, unsigned integer value.
- [MTLVertexFormat.uint2](https://developer.apple.com/documentation/metal/mtlvertexformat/uint2) — A two-component vector with 32-bit, unsigned integer values.
- [MTLVertexFormat.uint3](https://developer.apple.com/documentation/metal/mtlvertexformat/uint3) — A three-component vector with 32-bit, unsigned integer values.
- [MTLVertexFormat.uint4](https://developer.apple.com/documentation/metal/mtlvertexformat/uint4) — A four-component vector with 32-bit, unsigned integer values.

### 32-bit normalized integer formats
- [MTLVertexFormat.int1010102Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/int1010102normalized) — A four-component vector with 10-bit, normalized, signed integer values for red, green, and blue, and a 2-bit value for alpha.
- [MTLVertexFormat.uint1010102Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/uint1010102normalized) — A four-component vector with 10-bit, normalized, unsigned integer values for red, green, and blue, and a 2-bit value for alpha.
- [MTLVertexFormat.uchar4Normalized_bgra](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar4normalized_bgra) — A four-component vector with 8-bit, normalized, unsigned integer values for blue, green, red, and alpha.

### 16-bit floating-point formats
- [MTLVertexFormat.half](https://developer.apple.com/documentation/metal/mtlvertexformat/half) — A 16-bit floating-point value.
- [MTLVertexFormat.half2](https://developer.apple.com/documentation/metal/mtlvertexformat/half2) — A two-component vector with 16-bit floating-point values.
- [MTLVertexFormat.half3](https://developer.apple.com/documentation/metal/mtlvertexformat/half3) — A three-component vector with 16-bit floating-point values.
- [MTLVertexFormat.half4](https://developer.apple.com/documentation/metal/mtlvertexformat/half4) — A four-component vector with 16-bit floating-point values.

### 16-bit integer formats
- [MTLVertexFormat.short](https://developer.apple.com/documentation/metal/mtlvertexformat/short) — A 16-bit, signed integer value.
- [MTLVertexFormat.short2](https://developer.apple.com/documentation/metal/mtlvertexformat/short2) — A two-component vector with 16-bit, signed integer values.
- [MTLVertexFormat.short3](https://developer.apple.com/documentation/metal/mtlvertexformat/short3) — A three-component vector with 16-bit, signed integer values.
- [MTLVertexFormat.short4](https://developer.apple.com/documentation/metal/mtlvertexformat/short4) — A four-component vector with 16-bit, signed integer values.
- [MTLVertexFormat.ushort](https://developer.apple.com/documentation/metal/mtlvertexformat/ushort) — A 16-bit, unsigned integer value.
- [MTLVertexFormat.ushort2](https://developer.apple.com/documentation/metal/mtlvertexformat/ushort2) — A two-component vector with 16-bit, unsigned integer values.
- [MTLVertexFormat.ushort3](https://developer.apple.com/documentation/metal/mtlvertexformat/ushort3) — A three-component vector with 16-bit, unsigned integer values.
- [MTLVertexFormat.ushort4](https://developer.apple.com/documentation/metal/mtlvertexformat/ushort4) — A four-component vector with 16-bit, unsigned integer values.

### 16-bit normalized integer formats
- [MTLVertexFormat.shortNormalized](https://developer.apple.com/documentation/metal/mtlvertexformat/shortnormalized) — A 16-bit, normalized, signed integer value.
- [MTLVertexFormat.short2Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/short2normalized) — A two-component vector with 16-bit, normalized, signed integer values.
- [MTLVertexFormat.short3Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/short3normalized) — A three-component vector with 16-bit, normalized, signed integer values.
- [MTLVertexFormat.short4Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/short4normalized) — A four-component vector with 16-bit, normalized, signed integer values.
- [MTLVertexFormat.ushortNormalized](https://developer.apple.com/documentation/metal/mtlvertexformat/ushortnormalized) — A 16-bit, normalized, unsigned integer value.
- [MTLVertexFormat.ushort2Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/ushort2normalized) — A two-component vector with 16-bit, normalized, unsigned integer values.
- [MTLVertexFormat.ushort3Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/ushort3normalized) — A three-component vector with 16-bit, normalized, unsigned integer values.
- [MTLVertexFormat.ushort4Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/ushort4normalized) — A four-component vector with 16-bit, normalized, unsigned integer values.

### 8-bit integer formats
- [MTLVertexFormat.char](https://developer.apple.com/documentation/metal/mtlvertexformat/char) — An 8-bit, signed integer value.
- [MTLVertexFormat.char2](https://developer.apple.com/documentation/metal/mtlvertexformat/char2) — A two-component vector with 8-bit, signed integer values.
- [MTLVertexFormat.char3](https://developer.apple.com/documentation/metal/mtlvertexformat/char3) — A three-component vector with 8-bit, signed integer values.
- [MTLVertexFormat.char4](https://developer.apple.com/documentation/metal/mtlvertexformat/char4) — A four-component vector with 8-bit, signed integer values.
- [MTLVertexFormat.uchar](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar) — An 8-bit, unsigned integer value.
- [MTLVertexFormat.uchar2](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar2) — A two-component vector with 8-bit, unsigned integer values.
- [MTLVertexFormat.uchar3](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar3) — A three-component vector with 8-bit, unsigned integer values.
- [MTLVertexFormat.uchar4](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar4) — A four-component vector with 8-bit, unsigned integer values.

### 8-bit normalized integer formats
- [MTLVertexFormat.charNormalized](https://developer.apple.com/documentation/metal/mtlvertexformat/charnormalized) — An 8-bit, normalized, signed integer value.
- [MTLVertexFormat.char2Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/char2normalized) — A two-component vector with 8-bit, normalized, signed integer values.
- [MTLVertexFormat.char3Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/char3normalized) — A three-component vector with 8-bit, normalized, signed integer values.
- [MTLVertexFormat.char4Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/char4normalized) — A four-component vector with 8-bit, normalized, signed integer values.
- [MTLVertexFormat.ucharNormalized](https://developer.apple.com/documentation/metal/mtlvertexformat/ucharnormalized) — An 8-bit, normalized, unsigned integer value.
- [MTLVertexFormat.uchar2Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar2normalized) — A two-component vector with 8-bit, normalized, unsigned integer values.
- [MTLVertexFormat.uchar3Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar3normalized) — A three-component vector with 8-bit, normalized, unsigned integer values.
- [MTLVertexFormat.uchar4Normalized](https://developer.apple.com/documentation/metal/mtlvertexformat/uchar4normalized) — A four-component vector with 8-bit, normalized, unsigned integer values.

### Sentinel values
- [MTLVertexFormat.invalid](https://developer.apple.com/documentation/metal/mtlvertexformat/invalid) — A sentinel value that represents an empty set of vertex format options.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlvertexformat/init(rawvalue:)) — Creates a vertex format from a raw integer value.

## See also

### Organizing the vertex attribute
- [format](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/format) — The format of the vertex attribute.
- [offset](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/offset) — The location of an attribute in vertex data, determined by the byte offset from the start of the vertex data.
- [bufferIndex](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/bufferindex) — The index in the argument table for the associated vertex buffer.
