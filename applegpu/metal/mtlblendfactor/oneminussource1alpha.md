# MTLBlendFactor.oneMinusSource1Alpha

*Case · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.12, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblendfactor/oneminussource1alpha>

Blend factor of one minus source alpha. This option supports dual-source blending and reads from the second color output of the fragment function.

## Declaration

```swift
case oneMinusSource1Alpha
```

## Discussion

`F = 1 - Source.a`

## See also

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
