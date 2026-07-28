# MTLTensorDataType

*Enumeration · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensordatatype>

The possible data types for the elements of a tensor.

## Declaration

```swift
enum MTLTensorDataType
```

## Topics

### Enumeration Cases
- [MTLTensorDataType.bfloat16](https://developer.apple.com/documentation/metal/mtltensordatatype/bfloat16) — A 16-bit floating point data type with 8 exponent bits, 7 mantissa bits, and 1 sign bit.
- [MTLTensorDataType.float16](https://developer.apple.com/documentation/metal/mtltensordatatype/float16) — A half-precision floating point data type.
- [MTLTensorDataType.float32](https://developer.apple.com/documentation/metal/mtltensordatatype/float32) — A single-precision floating point data type.
- [MTLTensorDataType.int16](https://developer.apple.com/documentation/metal/mtltensordatatype/int16) — A 16-bit signed integer data type.
- [MTLTensorDataType.int2](https://developer.apple.com/documentation/metal/mtltensordatatype/int2) — A 2-bit signed integer data type.
- [MTLTensorDataType.int32](https://developer.apple.com/documentation/metal/mtltensordatatype/int32) — A 32-bit signed integer data type.
- [MTLTensorDataType.int4](https://developer.apple.com/documentation/metal/mtltensordatatype/int4) — A 4-bit signed integer data type.
- [MTLTensorDataType.int8](https://developer.apple.com/documentation/metal/mtltensordatatype/int8) — An 8-bit signed integer data type.
- [MTLTensorDataType.metalFloat4e2m1](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat4e2m1) — A 4-bit floating point data type with 2 exponent bits, 1 mantissa bit, and 1 sign bit.
- [MTLTensorDataType.metalFloat8e4m3](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8e4m3) — An 8-bit floating point data type with 4 exponent bits, 3 mantissa bits, and 1 sign bit.
- [MTLTensorDataType.metalFloat8e5m2](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8e5m2) — An 8-bit floating point data type with 5 exponent bits, 2 mantissa bits, and 1 sign bit.
- [MTLTensorDataType.metalFloat8ue8m0](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8ue8m0) — An 8-bit floating point data type with 8 exponent bits, 0 mantissa bits, and no sign bit.
- [MTLTensorDataType.none](https://developer.apple.com/documentation/metal/mtltensordatatype/none) — An invalid data type.
- [MTLTensorDataType.uint16](https://developer.apple.com/documentation/metal/mtltensordatatype/uint16) — A 16-bit unsigned integer data type.
- [MTLTensorDataType.uint2](https://developer.apple.com/documentation/metal/mtltensordatatype/uint2) — A 2-bit unsigned integer data type.
- [MTLTensorDataType.uint32](https://developer.apple.com/documentation/metal/mtltensordatatype/uint32) — A 32-bit unsigned integer data type.
- [MTLTensorDataType.uint4](https://developer.apple.com/documentation/metal/mtltensordatatype/uint4) — A 4-bit unsigned integer data type.
- [MTLTensorDataType.uint8](https://developer.apple.com/documentation/metal/mtltensordatatype/uint8) — An 8-bit unsigned integer data type.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtltensordatatype/init(rawvalue:))

## See also

### Tensors
- [MTLTensor](https://developer.apple.com/documentation/metal/mtltensor) — A resource representing a multi-dimensional array that you can use with machine learning workloads.
- [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor) — A configuration type for creating new tensor instances.
- [MTLTensorExtents](https://developer.apple.com/documentation/metal/mtltensorextents) — An integer array that holds per-dimension values such as tensor sizes, strides, or block factors
- [MTLTensorReferenceType](https://developer.apple.com/documentation/metal/mtltensorreferencetype) — An object that represents a tensor in the shading language in a struct or array.
- [MTLTensorUsage](https://developer.apple.com/documentation/metal/mtltensorusage) — The contexts in which you can use a tensor.
- [MTLTensorDomain](https://developer.apple.com/documentation/metal/mtltensordomain) — An error domain for errors that pertain to creating a tensor.
- [MTLTensorBinding](https://developer.apple.com/documentation/metal/mtltensorbinding) — An object that represents a tensor bound to a graphics or compute function or a machine learning function.
- [MTLTensorError](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct)
- [MTLTensorError.Code](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct/code) — The error codes that Metal can raise when you create a tensor.
- [MTLTensorDomain](https://developer.apple.com/documentation/metal/mtltensordomain) — An error domain for errors that pertain to creating a tensor.
- [MTL_TENSOR_MAX_RANK](https://developer.apple.com/documentation/metal/mtl_tensor_max_rank)
