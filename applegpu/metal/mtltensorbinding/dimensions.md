# dimensions

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensorbinding/dimensions>

The array of sizes, in elements, one for each dimension of this tensor.

## Declaration

```swift
var dimensions: MTLTensorExtents? { get }
```

## Discussion

For shader-bound tensors with dynamic extents, the [rank](https://developer.apple.com/documentation/metal/mtltensorextents/rank) of `dimensions` corresponds to the rank the shader function specifies, and [extentAtDimensionIndex:](https://developer.apple.com/documentation/metal/mtltensorextents/extentatdimensionindex:) always returns a value of -1.

For machine learning pipelines, `dimensions` corresponds to the default shape, if you provide one. Otherwise, it’s `nil` in the case of an undefined shape.
