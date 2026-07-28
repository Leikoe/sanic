# auxiliaryPlanes

*Instance Property · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensorreferencetype/auxiliaryplanes>

The auxiliary planes that this tensor reference requires.

## Declaration

```swift
var auxiliaryPlanes: [MTLTensorAuxiliaryPlaneType] { get }
```

## Discussion

Returns an array of [MTLTensorAuxiliaryPlaneType](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanetype) objects describing each auxiliary plane the shader expects. Empty if the tensor has no auxiliary planes.
