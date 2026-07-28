# usage

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensordescriptor/usage>

A set of contexts in which you can use tensors you create with this descriptor.

## Declaration

```swift
var usage: MTLTensorUsage { get set }
```

## Discussion

The default value for this property is a bitwise `OR` of:

- [render](https://developer.apple.com/documentation/metal/mtltensorusage/render)

- [compute](https://developer.apple.com/documentation/metal/mtltensorusage/compute)
