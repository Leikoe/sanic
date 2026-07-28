# MTLBlendFactor

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblendfactor>

The source and destination blend factors are often needed to complete specification of a blend operation. In most cases, the blend factor for both RGB values (*F(rgb)*) and alpha values (*F(a)*) are similar to one another, but in some cases, such as `MTLBlendFactorSourceAlphaSaturated`, the blend factor is slightly different. Four blend factors (`MTLBlendFactorBlendColor`, `MTLBlendFactorOneMinusBlendColor`, `MTLBlendFactorBlendAlpha`, and `MTLBlendFactorOneMinusBlendAlpha`) refer to a constant blend color value that is set by the [setBlendColor(red:green:blue:alpha:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setblendcolor(red:green:blue:alpha:)) method of `MTLRenderCommandEncoder`.

## Declaration

```swift
enum MTLBlendFactor
```

## Topics

### Blend factors
- [MTLBlendFactor.zero](https://developer.apple.com/documentation/metal/mtlblendfactor/zero) — Blend factor of zero.
- [MTLBlendFactor.one](https://developer.apple.com/documentation/metal/mtlblendfactor/one) — Blend factor of one.
- [MTLBlendFactor.sourceColor](https://developer.apple.com/documentation/metal/mtlblendfactor/sourcecolor) — Blend factor of source values.
- [MTLBlendFactor.oneMinusSourceColor](https://developer.apple.com/documentation/metal/mtlblendfactor/oneminussourcecolor) — Blend factor of one minus source values.
- [MTLBlendFactor.sourceAlpha](https://developer.apple.com/documentation/metal/mtlblendfactor/sourcealpha) — Blend factor of source alpha.
- [MTLBlendFactor.oneMinusSourceAlpha](https://developer.apple.com/documentation/metal/mtlblendfactor/oneminussourcealpha) — Blend factor of one minus source alpha.
- [MTLBlendFactor.destinationColor](https://developer.apple.com/documentation/metal/mtlblendfactor/destinationcolor) — Blend factor of destination values.
- [MTLBlendFactor.oneMinusDestinationColor](https://developer.apple.com/documentation/metal/mtlblendfactor/oneminusdestinationcolor) — Blend factor of one minus destination values.
- [MTLBlendFactor.destinationAlpha](https://developer.apple.com/documentation/metal/mtlblendfactor/destinationalpha) — Blend factor of destination alpha.
- [MTLBlendFactor.oneMinusDestinationAlpha](https://developer.apple.com/documentation/metal/mtlblendfactor/oneminusdestinationalpha) — Blend factor of one minus destination alpha.
- [MTLBlendFactor.sourceAlphaSaturated](https://developer.apple.com/documentation/metal/mtlblendfactor/sourcealphasaturated) — Blend factor of the minimum of either source alpha or one minus destination alpha.
- [MTLBlendFactor.blendColor](https://developer.apple.com/documentation/metal/mtlblendfactor/blendcolor) — A blend factor that applies the blend color’s red, green, and blue components.
- [MTLBlendFactor.oneMinusBlendColor](https://developer.apple.com/documentation/metal/mtlblendfactor/oneminusblendcolor) — A blend factor that applies one minus the blend color’s red, green, and blue components.
- [MTLBlendFactor.blendAlpha](https://developer.apple.com/documentation/metal/mtlblendfactor/blendalpha) — Blend factor of alpha value.
- [MTLBlendFactor.oneMinusBlendAlpha](https://developer.apple.com/documentation/metal/mtlblendfactor/oneminusblendalpha) — Blend factor of one minus alpha value.
- [MTLBlendFactor.source1Color](https://developer.apple.com/documentation/metal/mtlblendfactor/source1color) — Blend factor of source values. This option supports dual-source blending and reads from the second color output of the fragment function.
- [MTLBlendFactor.oneMinusSource1Color](https://developer.apple.com/documentation/metal/mtlblendfactor/oneminussource1color) — Blend factor of one minus source values. This option supports dual-source blending and reads from the second color output of the fragment function.
- [MTLBlendFactor.source1Alpha](https://developer.apple.com/documentation/metal/mtlblendfactor/source1alpha) — Blend factor of source alpha. This option supports dual-source blending and reads from the second color output of the fragment function.
- [MTLBlendFactor.oneMinusSource1Alpha](https://developer.apple.com/documentation/metal/mtlblendfactor/oneminussource1alpha) — Blend factor of one minus source alpha. This option supports dual-source blending and reads from the second color output of the fragment function.

### Enumeration Cases
- [MTLBlendFactor.unspecialized](https://developer.apple.com/documentation/metal/mtlblendfactor/unspecialized) — Defers assigning the blend factor.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlblendfactor/init(rawvalue:))

## See also

### Configuring blend factors
- [destinationAlphaBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/destinationalphablendfactor) — The destination blend factor (DBF) used by the alpha blend operation.
- [destinationRGBBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/destinationrgbblendfactor) — The destination blend factor (DBF) used by the RGB blend operation.
- [sourceAlphaBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/sourcealphablendfactor) — The source blend factor (SBF) used by the alpha blend operation.
- [sourceRGBBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/sourcergbblendfactor) — The source blend factor (SBF) used by the RGB blend operation.
