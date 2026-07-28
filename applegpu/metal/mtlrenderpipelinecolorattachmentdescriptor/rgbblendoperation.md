# rgbBlendOperation

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/rgbblendoperation>

The blend operation assigned for the RGB data.

## Declaration

```swift
var rgbBlendOperation: MTLBlendOperation { get set }
```

## Discussion

The default value is [MTLBlendOperation.add](https://developer.apple.com/documentation/metal/mtlblendoperation/add).

## See also

### Controlling blend operations
- [isBlendingEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/isblendingenabled) — A Boolean value that determines whether blending is enabled.
- [alphaBlendOperation](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/alphablendoperation) — The blend operation assigned for the alpha data.
- [MTLBlendOperation](https://developer.apple.com/documentation/metal/mtlblendoperation) — For every pixel, `MTLBlendOperation` determines how to combine and weight the source fragment values with the destination values. Some blend operations multiply the source values by a source blend factor (SBF), multiply the destination values by a destination blend factor (DBF), and then combine the results using addition or subtraction. Other blend operations use either a minimum or maximum function to determine the result.
