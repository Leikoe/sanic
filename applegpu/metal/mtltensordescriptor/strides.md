# strides

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensordescriptor/strides>

An array of strides, in elements, one for each dimension of this tensor, if applicable.

## Declaration

```swift
@NSCopying var strides: MTLTensorExtents? { get set }
```

## Discussion

The stride value of the array’s first element needs to be exactly `1`, because it is the innermost dimension. The strides for the subsequent dimensions can have different requirements based on the value of other properties. When the [usage](https://developer.apple.com/documentation/metal/mtltensordescriptor/usage) property includes the [machineLearning](https://developer.apple.com/documentation/metal/mtltensorusage/machinelearning) option:

- The second element of the array needs to be a multiple of 64 bytes.

- The rest of the elements in the array need to equal the product of the previous stride multiplied with the size of the previous dimension. For example: `strides[i] = strides[i - 1] * dimensions[i - 1]`.

When [dataType](https://developer.apple.com/documentation/metal/mtltensordescriptor/datatype) is [MTLTensorDataType.int2](https://developer.apple.com/documentation/metal/mtltensordatatype/int2), [MTLTensorDataType.uint2](https://developer.apple.com/documentation/metal/mtltensordatatype/uint2), [MTLTensorDataType.int4](https://developer.apple.com/documentation/metal/mtltensordatatype/int4), [MTLTensorDataType.uint4](https://developer.apple.com/documentation/metal/mtltensordatatype/uint4), [MTLTensorDataType.metalFloat4e2m1](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat4e2m1), [MTLTensorDataType.metalFloat8e4m3](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8e4m3), [MTLTensorDataType.metalFloat8e5m2](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8e5m2), or [MTLTensorDataType.metalFloat8ue8m0](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8ue8m0), all elements of the array, except for the first element, need to be a multiple of 128 bytes.

> **Tip:**
> You can improve runtime performance by using strides that are multiples of 128, even when it’s not a requirement.

Only set this property when creating tensors from a buffer.
