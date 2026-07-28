# setDescriptor(_:for:)

*Instance Method · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptormap/setdescriptor(_:for:)>

Sets the auxiliary plane descriptor for the given plane type.

## Declaration

```swift
func setDescriptor(_ descriptor: MTLTensorAuxiliaryPlaneDescriptor, for plane: MTLTensorPlaneType)
```

## Parameters

- **descriptor** — The descriptor configuring the auxiliary plane.
- **plane** — The plane type to associate the descriptor with.

## Discussion

[MTLTensorPlaneType.data](https://developer.apple.com/documentation/metal/mtltensorplanetype/data) is not a valid plane type for this method. The data plane is always present, and you configure it directly on [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor).

[MTLTensorPlaneType.scales](https://developer.apple.com/documentation/metal/mtltensorplanetype/scales) auxiliary planes only support [MTLTensorDataType.metalFloat8ue8m0](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8ue8m0) as a data type.
