# MTLTensorAuxiliaryPlane

*Protocol · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensorauxiliaryplane>

A type that represents the configuration and storage of an auxiliary plane in a multi-plane tensor.

## Declaration

```swift
protocol MTLTensorAuxiliaryPlane : NSObjectProtocol
```

## Topics

### Instance Properties
- [blockFactors](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplane/blockfactors) — The number of data plane elements that correspond to one element in this auxiliary plane.
- [buffer](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplane/buffer) — The buffer that provides the underlying storage for this plane, or `nil` if no buffer was provided at initialization.
- [bufferOffset](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplane/bufferoffset) — The byte offset into [buffer](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplane/buffer) where this plane’s data begins, or `0` if no buffer was provided at initialization.
- [dataType](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplane/datatype) — The data format of all elements in the plane.
- [planeType](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplane/planetype) — The type of information this plane stores.
