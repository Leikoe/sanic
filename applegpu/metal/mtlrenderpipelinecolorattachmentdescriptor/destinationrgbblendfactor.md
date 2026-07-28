# destinationRGBBlendFactor

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/destinationrgbblendfactor>

The destination blend factor (DBF) used by the RGB blend operation.

## Declaration

```swift
var destinationRGBBlendFactor: MTLBlendFactor { get set }
```

## Discussion

The default value is [MTLBlendFactor.zero](https://developer.apple.com/documentation/metal/mtlblendfactor/zero).

## See also

### Configuring blend factors
- [destinationAlphaBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/destinationalphablendfactor) — The destination blend factor (DBF) used by the alpha blend operation.
- [sourceAlphaBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/sourcealphablendfactor) — The source blend factor (SBF) used by the alpha blend operation.
- [sourceRGBBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/sourcergbblendfactor) — The source blend factor (SBF) used by the RGB blend operation.
- [MTLBlendFactor](https://developer.apple.com/documentation/metal/mtlblendfactor) — The source and destination blend factors are often needed to complete specification of a blend operation. In most cases, the blend factor for both RGB values (*F(rgb)*) and alpha values (*F(a)*) are similar to one another, but in some cases, such as `MTLBlendFactorSourceAlphaSaturated`, the blend factor is slightly different. Four blend factors (`MTLBlendFactorBlendColor`, `MTLBlendFactorOneMinusBlendColor`, `MTLBlendFactorBlendAlpha`, and `MTLBlendFactorOneMinusBlendAlpha`) refer to a constant blend color value that is set by the [setBlendColor(red:green:blue:alpha:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setblendcolor(red:green:blue:alpha:)) method of `MTLRenderCommandEncoder`.
