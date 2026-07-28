# strides

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensor/strides>

An array of strides, in elements, one for each dimension of this tensor, if applicable.

## Declaration

```swift
var strides: MTLTensorExtents? { get }
```

## Discussion

This property is non-nil only for tensors created from a buffer.
