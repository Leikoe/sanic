# elementTensorReferenceType()

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlarraytype/elementtensorreferencetype()>

Provides a description of the underlying tensor type when this array holds tensors as its elements.

## Declaration

```swift
func elementTensorReferenceType() -> MTLTensorReferenceType?
```

## Return Value

A description of the tensor type that this array holds, or `nil` if this struct member doesn’t hold a tensor.
