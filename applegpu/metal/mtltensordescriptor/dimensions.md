# dimensions

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensordescriptor/dimensions>

An array of sizes, in elements, one for each dimension of the tensors you create with this descriptor.

## Declaration

```swift
@NSCopying var dimensions: MTLTensorExtents { get set }
```

## Discussion

Every element of the array needs to be greater than `0`.

When [dataType](https://developer.apple.com/documentation/metal/mtltensordescriptor/datatype) is [MTLTensorDataType.int2](https://developer.apple.com/documentation/metal/mtltensordatatype/int2), [MTLTensorDataType.uint2](https://developer.apple.com/documentation/metal/mtltensordatatype/uint2), [MTLTensorDataType.int4](https://developer.apple.com/documentation/metal/mtltensordatatype/int4), [MTLTensorDataType.uint4](https://developer.apple.com/documentation/metal/mtltensordatatype/uint4), [MTLTensorDataType.metalFloat4e2m1](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat4e2m1), [MTLTensorDataType.metalFloat8e4m3](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8e4m3), [MTLTensorDataType.metalFloat8e5m2](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8e5m2), or [MTLTensorDataType.metalFloat8ue8m0](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8ue8m0):

- The dimension value of the array’s first element needs to be a multiple of 32 elements.

- The extents needs to have at least one dimension.

If the tensor has auxiliary planes, each dimension needs to be evenly divisible by its corresponding block factor.

The default value of this property is a rank one extents with size one.
