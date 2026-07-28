# MTLBlendOperation

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblendoperation>

For every pixel, `MTLBlendOperation` determines how to combine and weight the source fragment values with the destination values. Some blend operations multiply the source values by a source blend factor (SBF), multiply the destination values by a destination blend factor (DBF), and then combine the results using addition or subtraction. Other blend operations use either a minimum or maximum function to determine the result.

## Declaration

```swift
enum MTLBlendOperation
```

## Topics

### Blend operations
- [MTLBlendOperation.add](https://developer.apple.com/documentation/metal/mtlblendoperation/add) — Add portions of both source and destination pixel values.
- [MTLBlendOperation.subtract](https://developer.apple.com/documentation/metal/mtlblendoperation/subtract) — Subtract a portion of the destination pixel values from a portion of the source.
- [MTLBlendOperation.reverseSubtract](https://developer.apple.com/documentation/metal/mtlblendoperation/reversesubtract) — Subtract a portion of the source values from a portion of the destination pixel values.
- [MTLBlendOperation.min](https://developer.apple.com/documentation/metal/mtlblendoperation/min) — Minimum of the source and destination pixel values.
- [MTLBlendOperation.max](https://developer.apple.com/documentation/metal/mtlblendoperation/max) — Maximum of the source and destination pixel values.

### Enumeration Cases
- [MTLBlendOperation.unspecialized](https://developer.apple.com/documentation/metal/mtlblendoperation/unspecialized) — Defers assigning the blend operation.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlblendoperation/init(rawvalue:))

## See also

### Controlling blend operations
- [isBlendingEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/isblendingenabled) — A Boolean value that determines whether blending is enabled.
- [alphaBlendOperation](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/alphablendoperation) — The blend operation assigned for the alpha data.
- [rgbBlendOperation](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/rgbblendoperation) — The blend operation assigned for the RGB data.
