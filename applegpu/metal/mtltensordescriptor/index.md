# MTLTensorDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensordescriptor>

A configuration type for creating new tensor instances.

## Declaration

```swift
class MTLTensorDescriptor
```

## Topics

### Instance Properties
- [auxiliaryPlanes](https://developer.apple.com/documentation/metal/mtltensordescriptor/auxiliaryplanes) — The auxiliary plane configurations for this tensor.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtltensordescriptor/cpucachemode) — A value that configures the cache mode of CPU mapping of tensors you create with this descriptor.
- [dataType](https://developer.apple.com/documentation/metal/mtltensordescriptor/datatype) — The data format of all elements in the data plane.
- [dimensions](https://developer.apple.com/documentation/metal/mtltensordescriptor/dimensions) — An array of sizes, in elements, one for each dimension of the tensors you create with this descriptor.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtltensordescriptor/hazardtrackingmode) — A value that configures the hazard tracking of tensors you create with this descriptor.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtltensordescriptor/resourceoptions) — A packed set of the [storageMode](https://developer.apple.com/documentation/metal/mtltensordescriptor/storagemode), [cpuCacheMode](https://developer.apple.com/documentation/metal/mtltensordescriptor/cpucachemode), and [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtltensordescriptor/hazardtrackingmode) properties.
- [storageMode](https://developer.apple.com/documentation/metal/mtltensordescriptor/storagemode) — A value that configures the memory location and access permissions of tensors you create with this descriptor.
- [strides](https://developer.apple.com/documentation/metal/mtltensordescriptor/strides) — An array of strides, in elements, one for each dimension of this tensor, if applicable.
- [usage](https://developer.apple.com/documentation/metal/mtltensordescriptor/usage) — A set of contexts in which you can use tensors you create with this descriptor.

## See also

### Tensors
- [MTLTensor](https://developer.apple.com/documentation/metal/mtltensor) — A resource representing a multi-dimensional array that you can use with machine learning workloads.
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
