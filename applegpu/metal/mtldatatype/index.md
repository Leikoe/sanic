# MTLDataType

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldatatype>

The parameter type options for GPU functions, such as shaders and compute kernels.

## Declaration

```swift
enum MTLDataType
```

## Overview

Metal reports or accepts this type in several reflection and configuration contexts, such as:

- The [type](https://developer.apple.com/documentation/metal/mtlfunctionconstant/type) property of [MTLFunctionConstant](https://developer.apple.com/documentation/metal/mtlfunctionconstant)

- The [attributeType](https://developer.apple.com/documentation/metal/mtlattribute/attributetype) property of [MTLAttribute](https://developer.apple.com/documentation/metal/mtlattribute)

- The [attributeType](https://developer.apple.com/documentation/metal/mtlvertexattribute/attributetype) property of [MTLVertexAttribute](https://developer.apple.com/documentation/metal/mtlvertexattribute)

- The [setConstantValue(_:type:withName:)](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalue(_:type:withname:)) method of [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues)

### Normalized integer types

Color types with `Snorm` in the name are normalized signed integer types. For these types, values in the range `[-1.0, 1.0]` map to `[MIN_INT, MAX_INT]`, where `MIN_INT` is the most negative integer and `MAX_INT` is the most positive integer for the number of bits in the storage size. Positive values and zero distribute uniformly in the range `[0.0, 1.0]`, and negative integer values greater than `(MIN_INT + 1)` distribute uniformly in the range `(-1.0, 0.0)`.

> **Important:**
>  For normalized signed integer types, the values `MIN_INT` and `(MIN_INT + 1)` both map to `-1.0`.

Color types with `Unorm` in the name are normalized unsigned integer types. For these types, values in the range `[0.0, 1.0]` map to `[0, MAX_UINT]`, where `MAX_UINT` is the largest unsigned integer for the number of bits in the storage size.

Metal stores data in little-endian byte order, with the least-significant byte at the lowest memory address. Formats with multibyte components also store each component in little-endian byte order.

## Topics

### 64-bit integer types
- [MTLDataType.long](https://developer.apple.com/documentation/metal/mtldatatype/long) — A 64-bit, signed integer value.
- [MTLDataType.long2](https://developer.apple.com/documentation/metal/mtldatatype/long2) — A two-component vector with 64-bit, signed integer values.
- [MTLDataType.long3](https://developer.apple.com/documentation/metal/mtldatatype/long3) — A three-component vector with 64-bit, signed integer values.
- [MTLDataType.long4](https://developer.apple.com/documentation/metal/mtldatatype/long4) — A four-component vector with 64-bit, signed integer values.
- [MTLDataType.ulong](https://developer.apple.com/documentation/metal/mtldatatype/ulong) — A 64-bit, unsigned integer value.
- [MTLDataType.ulong2](https://developer.apple.com/documentation/metal/mtldatatype/ulong2) — A two-component vector with 64-bit, unsigned integer values.
- [MTLDataType.ulong3](https://developer.apple.com/documentation/metal/mtldatatype/ulong3) — A three-component vector with 64-bit, unsigned integer values.
- [MTLDataType.ulong4](https://developer.apple.com/documentation/metal/mtldatatype/ulong4) — A four-component vector with 64-bit, unsigned integer values.

### 64-bit color integer types
- [MTLDataType.rgba16Snorm](https://developer.apple.com/documentation/metal/mtldatatype/rgba16snorm) — An ordinary pixel with four components, each of which is a 16-bit, normalized, signed integer value.
- [MTLDataType.rgba16Unorm](https://developer.apple.com/documentation/metal/mtldatatype/rgba16unorm) — An ordinary pixel with four components, each of which is a 16-bit, normalized, unsigned integer value.

### 32-bit floating-point types
- [MTLDataType.float](https://developer.apple.com/documentation/metal/mtldatatype/float) — A 32-bit floating-point value.
- [MTLDataType.float2](https://developer.apple.com/documentation/metal/mtldatatype/float2) — A two-component vector with 32-bit floating-point values.
- [MTLDataType.float3](https://developer.apple.com/documentation/metal/mtldatatype/float3) — A three-component vector with 32-bit floating-point values.
- [MTLDataType.float4](https://developer.apple.com/documentation/metal/mtldatatype/float4) — A four-component vector with 32-bit floating-point values.

### 32-bit floating-point matrix types
- [MTLDataType.float2x2](https://developer.apple.com/documentation/metal/mtldatatype/float2x2) — A 2x2 component matrix with 32-bit floating-point values.
- [MTLDataType.float2x3](https://developer.apple.com/documentation/metal/mtldatatype/float2x3) — A 2x3 component matrix with 32-bit floating-point values.
- [MTLDataType.float2x4](https://developer.apple.com/documentation/metal/mtldatatype/float2x4) — A 2x4 component matrix with 32-bit floating-point values.
- [MTLDataType.float3x2](https://developer.apple.com/documentation/metal/mtldatatype/float3x2) — A 3x2 component matrix with 32-bit floating-point values.
- [MTLDataType.float3x3](https://developer.apple.com/documentation/metal/mtldatatype/float3x3) — A 3x3 component matrix with 32-bit floating-point values.
- [MTLDataType.float3x4](https://developer.apple.com/documentation/metal/mtldatatype/float3x4) — A 3x4 component matrix with 32-bit floating-point values.
- [MTLDataType.float4x2](https://developer.apple.com/documentation/metal/mtldatatype/float4x2) — A 4x2 component matrix with 32-bit floating-point values.
- [MTLDataType.float4x3](https://developer.apple.com/documentation/metal/mtldatatype/float4x3) — A 4x3 component matrix with 32-bit floating-point values.
- [MTLDataType.float4x4](https://developer.apple.com/documentation/metal/mtldatatype/float4x4) — A 4x4 component matrix with 32-bit floating-point values.

### 32-bit color floating-point types
- [MTLDataType.rgb9e5Float](https://developer.apple.com/documentation/metal/mtldatatype/rgb9e5float) — A packed 32-bit format with three color components, each of which is a 9-bit floating-point value.
- [MTLDataType.rg11b10Float](https://developer.apple.com/documentation/metal/mtldatatype/rg11b10float) — A packed 32-bit format with three floating-point color components, two of which are 11-bit values, and one is a 10-bit value.

### 32-bit color integer types
- [MTLDataType.rgba8Snorm](https://developer.apple.com/documentation/metal/mtldatatype/rgba8snorm) — An ordinary pixel with four components, each of which is an 8-bit, normalized, signed integer value.
- [MTLDataType.rgba8Unorm](https://developer.apple.com/documentation/metal/mtldatatype/rgba8unorm) — An ordinary pixel with four components, each of which is an 8-bit, normalized, unsigned integer value.
- [MTLDataType.rgba8Unorm_srgb](https://developer.apple.com/documentation/metal/mtldatatype/rgba8unorm_srgb) — An ordinary pixel with four components, each of which is an 8-bit, normalized, unsigned integer value in the sRGB color space.
- [MTLDataType.rg16Snorm](https://developer.apple.com/documentation/metal/mtldatatype/rg16snorm) — An ordinary pixel with two components, each of which is a 16-bit, normalized, signed integer value.
- [MTLDataType.rg16Unorm](https://developer.apple.com/documentation/metal/mtldatatype/rg16unorm) — An ordinary pixel with two components, each of which is a 16-bit, normalized, unsigned integer value.
- [MTLDataType.rgb10a2Unorm](https://developer.apple.com/documentation/metal/mtldatatype/rgb10a2unorm) — A packed 32-bit format with three color components, each of which is a 10-bit, normalized, unsigned integer value.

### 32-bit integer types
- [MTLDataType.int](https://developer.apple.com/documentation/metal/mtldatatype/int) — A 32-bit, signed integer value.
- [MTLDataType.int2](https://developer.apple.com/documentation/metal/mtldatatype/int2) — A two-component vector with 32-bit, signed integer values.
- [MTLDataType.int3](https://developer.apple.com/documentation/metal/mtldatatype/int3) — A three-component vector with 32-bit, signed integer values.
- [MTLDataType.int4](https://developer.apple.com/documentation/metal/mtldatatype/int4) — A four-component vector with 32-bit, signed integer values.
- [MTLDataType.uint](https://developer.apple.com/documentation/metal/mtldatatype/uint) — A 32-bit, unsigned integer value.
- [MTLDataType.uint2](https://developer.apple.com/documentation/metal/mtldatatype/uint2) — A two-component vector with 32-bit, unsigned integer values.
- [MTLDataType.uint3](https://developer.apple.com/documentation/metal/mtldatatype/uint3) — A three-component vector with 32-bit, unsigned integer values.
- [MTLDataType.uint4](https://developer.apple.com/documentation/metal/mtldatatype/uint4) — A four-component vector with 32-bit, unsigned integer values.

### 16-bit floating-point types
- [MTLDataType.half](https://developer.apple.com/documentation/metal/mtldatatype/half) — A 16-bit floating-point value.
- [MTLDataType.half2](https://developer.apple.com/documentation/metal/mtldatatype/half2) — A two-component vector with 16-bit floating-point values.
- [MTLDataType.half3](https://developer.apple.com/documentation/metal/mtldatatype/half3) — A three-component vector with 16-bit floating-point values.
- [MTLDataType.half4](https://developer.apple.com/documentation/metal/mtldatatype/half4) — A four-component vector with 16-bit floating-point values.

### 16-bit floating-point matrix types
- [MTLDataType.half2x2](https://developer.apple.com/documentation/metal/mtldatatype/half2x2) — A 2x2 component matrix with 16-bit floating-point values.
- [MTLDataType.half2x3](https://developer.apple.com/documentation/metal/mtldatatype/half2x3) — A 2x3 component matrix with 16-bit floating-point values.
- [MTLDataType.half2x4](https://developer.apple.com/documentation/metal/mtldatatype/half2x4) — A 2x4 component matrix with 16-bit floating-point values.
- [MTLDataType.half3x2](https://developer.apple.com/documentation/metal/mtldatatype/half3x2) — A 3x2 component matrix with 16-bit floating-point values.
- [MTLDataType.half3x3](https://developer.apple.com/documentation/metal/mtldatatype/half3x3) — A 3x3 component matrix with 16-bit floating-point values.
- [MTLDataType.half3x4](https://developer.apple.com/documentation/metal/mtldatatype/half3x4) — A 3x4 component matrix with 16-bit floating-point values.
- [MTLDataType.half4x2](https://developer.apple.com/documentation/metal/mtldatatype/half4x2) — A 4x2 component matrix with 16-bit floating-point values.
- [MTLDataType.half4x3](https://developer.apple.com/documentation/metal/mtldatatype/half4x3) — A 4x3 component matrix with 16-bit floating-point values.
- [MTLDataType.half4x4](https://developer.apple.com/documentation/metal/mtldatatype/half4x4) — A 4x4 component matrix with 16-bit floating-point values.

### 16-bit brain floating-point types
- [MTLDataType.bfloat](https://developer.apple.com/documentation/metal/mtldatatype/bfloat) — A 16-bit, brain floating-point value.
- [MTLDataType.bfloat2](https://developer.apple.com/documentation/metal/mtldatatype/bfloat2) — A two-component vector with 16-bit, brain floating-point values.
- [MTLDataType.bfloat3](https://developer.apple.com/documentation/metal/mtldatatype/bfloat3) — A three-component vector with 16-bit, brain floating-point values.
- [MTLDataType.bfloat4](https://developer.apple.com/documentation/metal/mtldatatype/bfloat4) — A four-component vector with 16-bit, brain floating-point values.

### 16-bit integer types
- [MTLDataType.short](https://developer.apple.com/documentation/metal/mtldatatype/short) — A 16-bit, signed integer value.
- [MTLDataType.short2](https://developer.apple.com/documentation/metal/mtldatatype/short2) — A two-component vector with 16-bit, signed integer values.
- [MTLDataType.short3](https://developer.apple.com/documentation/metal/mtldatatype/short3) — A three-component vector with 16-bit, signed integer values.
- [MTLDataType.short4](https://developer.apple.com/documentation/metal/mtldatatype/short4) — A four-component vector with 16-bit, signed integer values.
- [MTLDataType.ushort](https://developer.apple.com/documentation/metal/mtldatatype/ushort) — A 16-bit, unsigned integer value.
- [MTLDataType.ushort2](https://developer.apple.com/documentation/metal/mtldatatype/ushort2) — A two-component vector with 16-bit, unsigned integer values.
- [MTLDataType.ushort3](https://developer.apple.com/documentation/metal/mtldatatype/ushort3) — A three-component vector with 16-bit, unsigned integer values.
- [MTLDataType.ushort4](https://developer.apple.com/documentation/metal/mtldatatype/ushort4) — A four-component vector with 16-bit, unsigned integer values.

### 16-bit color integer types
- [MTLDataType.rg8Snorm](https://developer.apple.com/documentation/metal/mtldatatype/rg8snorm) — An ordinary pixel with two components, each of which is an 8-bit, normalized, signed integer value.
- [MTLDataType.rg8Unorm](https://developer.apple.com/documentation/metal/mtldatatype/rg8unorm) — An ordinary pixel with two components, each of which is an 8-bit, normalized, unsigned integer value.
- [MTLDataType.r16Snorm](https://developer.apple.com/documentation/metal/mtldatatype/r16snorm) — An ordinary pixel with one component that’s a 16-bit, normalized, signed integer value.
- [MTLDataType.r16Unorm](https://developer.apple.com/documentation/metal/mtldatatype/r16unorm) — An ordinary pixel with one component that’s a 16-bit, normalized, unsigned integer value.

### 8-bit integer types
- [MTLDataType.char](https://developer.apple.com/documentation/metal/mtldatatype/char) — An 8-bit, signed integer value.
- [MTLDataType.char2](https://developer.apple.com/documentation/metal/mtldatatype/char2) — A two-component vector with 8-bit, signed integer values.
- [MTLDataType.char3](https://developer.apple.com/documentation/metal/mtldatatype/char3) — A three-component vector with 8-bit, signed integer values.
- [MTLDataType.char4](https://developer.apple.com/documentation/metal/mtldatatype/char4) — A four-component vector with 8-bit, signed integer values.
- [MTLDataType.uchar](https://developer.apple.com/documentation/metal/mtldatatype/uchar) — An 8-bit, unsigned integer value.
- [MTLDataType.uchar2](https://developer.apple.com/documentation/metal/mtldatatype/uchar2) — A two-component vector with 8-bit, unsigned integer values.
- [MTLDataType.uchar3](https://developer.apple.com/documentation/metal/mtldatatype/uchar3) — A three-component vector with 8-bit, unsigned integer values.
- [MTLDataType.uchar4](https://developer.apple.com/documentation/metal/mtldatatype/uchar4) — A four-component vector with 8-bit, unsigned integer values.

### 8-bit color integer types
- [MTLDataType.r8Snorm](https://developer.apple.com/documentation/metal/mtldatatype/r8snorm) — An ordinary pixel with one component that’s an 8-bit, normalized, signed integer value.
- [MTLDataType.r8Unorm](https://developer.apple.com/documentation/metal/mtldatatype/r8unorm) — An ordinary pixel with one component that’s an 8-bit, normalized, unsigned integer value.

### Boolean types
- [MTLDataType.bool](https://developer.apple.com/documentation/metal/mtldatatype/bool) — A Boolean value.
- [MTLDataType.bool2](https://developer.apple.com/documentation/metal/mtldatatype/bool2) — A two-component Boolean vector.
- [MTLDataType.bool3](https://developer.apple.com/documentation/metal/mtldatatype/bool3) — A three-component Boolean vector.
- [MTLDataType.bool4](https://developer.apple.com/documentation/metal/mtldatatype/bool4) — A four-component Boolean vector.

### Resource types
- [MTLDataType.tensor](https://developer.apple.com/documentation/metal/mtldatatype/tensor) — Represents a data type corresponding to a machine learning tensor.
- [MTLDataType.sampler](https://developer.apple.com/documentation/metal/mtldatatype/sampler) — A Metal texture sampler instance.
- [MTLDataType.texture](https://developer.apple.com/documentation/metal/mtldatatype/texture) — A Metal texture resource instance.
- [MTLDataType.renderPipeline](https://developer.apple.com/documentation/metal/mtldatatype/renderpipeline) — A Metal render pipeline instance.
- [MTLDataType.computePipeline](https://developer.apple.com/documentation/metal/mtldatatype/computepipeline) — A Metal compute pipeline instance.
- [MTLDataType.depthStencilState](https://developer.apple.com/documentation/metal/mtldatatype/depthstencilstate) — Represents a data type corresponding to a depth-stencil state object.
- [MTLDataType.indirectCommandBuffer](https://developer.apple.com/documentation/metal/mtldatatype/indirectcommandbuffer) — An indirect command buffer resource instance.
- [MTLDataType.visibleFunctionTable](https://developer.apple.com/documentation/metal/mtldatatype/visiblefunctiontable) — A table of visible functions that a render or compute pipeline can call.
- [MTLDataType.intersectionFunctionTable](https://developer.apple.com/documentation/metal/mtldatatype/intersectionfunctiontable) — A table of intersection functions that a render or compute pipeline can call.
- [MTLDataType.primitiveAccelerationStructure](https://developer.apple.com/documentation/metal/mtldatatype/primitiveaccelerationstructure) — A low-level ray-tracing acceleration structure for a set of primitives.
- [MTLDataType.instanceAccelerationStructure](https://developer.apple.com/documentation/metal/mtldatatype/instanceaccelerationstructure) — A high-level, ray-tracing acceleration structure for a set of low-level primitive instances.

### Collection types
- [MTLDataType.struct](https://developer.apple.com/documentation/metal/mtldatatype/struct) — A structure instance.
- [MTLDataType.array](https://developer.apple.com/documentation/metal/mtldatatype/array) — An array instance.
- [MTLDataType.pointer](https://developer.apple.com/documentation/metal/mtldatatype/pointer) — A pointer.

### Sentinel values
- [MTLDataType.none](https://developer.apple.com/documentation/metal/mtldatatype/none) — A sentinel value that represents a GPU function parameter that doesn’t have a valid data type.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtldatatype/init(rawvalue:)) — Creates a data type instance from a raw integer value.

## See also

### Shader types
- [MTLType](https://developer.apple.com/documentation/metal/mtltype) — A description of a data type.
- [MTLArrayType](https://developer.apple.com/documentation/metal/mtlarraytype) — A description of an array.
- [MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) — A description of a structure.
- [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) — An instance that provides information about a field in a structure.
- [MTLPointerType](https://developer.apple.com/documentation/metal/mtlpointertype) — A description of a pointer.
- [MTLTextureReferenceType](https://developer.apple.com/documentation/metal/mtltexturereferencetype) — A description of a texture.
