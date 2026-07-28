# MTLTensor

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensor>

A resource representing a multi-dimensional array that you can use with machine learning workloads.

## Declaration

```swift
protocol MTLTensor : MTLResource
```

## Topics

### Instance Properties
- [auxiliaryPlanes](https://developer.apple.com/documentation/metal/mtltensor/auxiliaryplanes) — The auxiliary planes of this tensor.
- [buffer](https://developer.apple.com/documentation/metal/mtltensor/buffer) — A buffer instance this tensor shares its storage with or `nil` if this tensor does not wrap an underlying buffer.
- [bufferOffset](https://developer.apple.com/documentation/metal/mtltensor/bufferoffset) — An offset, in bytes, into the buffer instance this tensor shares its storage with, or zero if this tensor does not wrap an underlying buffer.
- [dataType](https://developer.apple.com/documentation/metal/mtltensor/datatype) — The underlying data format of the data plane.
- [dimensions](https://developer.apple.com/documentation/metal/mtltensor/dimensions) — An array of sizes, in elements, one for each dimension of this tensor.
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtltensor/gpuresourceid) — A handle that represents the GPU resource, which you can store in an argument buffer.
- [strides](https://developer.apple.com/documentation/metal/mtltensor/strides) — An array of strides, in elements, one for each dimension of this tensor, if applicable.
- [usage](https://developer.apple.com/documentation/metal/mtltensor/usage) — A set of contexts in which you can use this tensor.

### Instance Methods
- [getBytes(_:strides:sliceOrigin:sliceDimensions:)](https://developer.apple.com/documentation/metal/mtltensor/getbytes(_:strides:sliceorigin:slicedimensions:)) — Copies data from a slice of the data plane of this tensor into a pointer you provide.
- [getBytes(_:strides:sliceOrigin:sliceDimensions:plane:)](https://developer.apple.com/documentation/metal/mtltensor/getbytes(_:strides:sliceorigin:slicedimensions:plane:)) — Copies data from a slice of a plane of this tensor into a pointer you provide.
- [replace(sliceOrigin:sliceDimensions:plane:withBytes:strides:)](https://developer.apple.com/documentation/metal/mtltensor/replace(sliceorigin:slicedimensions:plane:withbytes:strides:)) — Replaces a slice of a plane of this tensor with data from a pointer you provide.
- [replace(sliceOrigin:sliceDimensions:withBytes:strides:)](https://developer.apple.com/documentation/metal/mtltensor/replace(sliceorigin:slicedimensions:withbytes:strides:)) — Replaces a slice of the data plane of this tensor with data from a pointer you provide.

## See also

### Tensors
- [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor) — A configuration type for creating new tensor instances.
- [MTLTensorExtents](https://developer.apple.com/documentation/metal/mtltensorextents) — An integer array that holds per-dimension values such as tensor sizes, strides, or block factors
- [MTLTensorReferenceType](https://developer.apple.com/documentation/metal/mtltensorreferencetype) — An object that represents a tensor in the shading language in a struct or array.
- [MTLTensorUsage](https://developer.apple.com/documentation/metal/mtltensorusage) — The contexts in which you can use a tensor.
- [MTLTensorDomain](https://developer.apple.com/documentation/metal/mtltensordomain) — An error domain for errors that pertain to creating a tensor.
- [MTLTensorBinding](https://developer.apple.com/documentation/metal/mtltensorbinding) — An object that represents a tensor bound to a graphics or compute function or a machine learning function.
- [MTLTensorError](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct)
- [MTLTensorError.Code](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct/code) — The error codes that Metal can raise when you create a tensor.
- [MTLTensorDataType](https://developer.apple.com/documentation/metal/mtltensordatatype) — The possible data types for the elements of a tensor.
- [MTLTensorDomain](https://developer.apple.com/documentation/metal/mtltensordomain) — An error domain for errors that pertain to creating a tensor.
- [MTL_TENSOR_MAX_RANK](https://developer.apple.com/documentation/metal/mtl_tensor_max_rank)
