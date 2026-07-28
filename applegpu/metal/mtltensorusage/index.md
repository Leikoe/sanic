# MTLTensorUsage

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensorusage>

The contexts in which you can use a tensor.

## Declaration

```swift
struct MTLTensorUsage
```

## Topics

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtltensorusage/init(rawvalue:))

### Type Properties
- [compute](https://developer.apple.com/documentation/metal/mtltensorusage/compute) — A tensor context that applies to compute encoders.
- [machineLearning](https://developer.apple.com/documentation/metal/mtltensorusage/machinelearning) — A tensor context that applies to machine learning encoders.
- [render](https://developer.apple.com/documentation/metal/mtltensorusage/render) — A tensor context that applies to render encoders.

## See also

### Tensors
- [MTLTensor](https://developer.apple.com/documentation/metal/mtltensor) — A resource representing a multi-dimensional array that you can use with machine learning workloads.
- [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor) — A configuration type for creating new tensor instances.
- [MTLTensorExtents](https://developer.apple.com/documentation/metal/mtltensorextents) — An integer array that holds per-dimension values such as tensor sizes, strides, or block factors
- [MTLTensorReferenceType](https://developer.apple.com/documentation/metal/mtltensorreferencetype) — An object that represents a tensor in the shading language in a struct or array.
- [MTLTensorDomain](https://developer.apple.com/documentation/metal/mtltensordomain) — An error domain for errors that pertain to creating a tensor.
- [MTLTensorBinding](https://developer.apple.com/documentation/metal/mtltensorbinding) — An object that represents a tensor bound to a graphics or compute function or a machine learning function.
- [MTLTensorError](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct)
- [MTLTensorError.Code](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct/code) — The error codes that Metal can raise when you create a tensor.
- [MTLTensorDataType](https://developer.apple.com/documentation/metal/mtltensordatatype) — The possible data types for the elements of a tensor.
- [MTLTensorDomain](https://developer.apple.com/documentation/metal/mtltensordomain) — An error domain for errors that pertain to creating a tensor.
- [MTL_TENSOR_MAX_RANK](https://developer.apple.com/documentation/metal/mtl_tensor_max_rank)
