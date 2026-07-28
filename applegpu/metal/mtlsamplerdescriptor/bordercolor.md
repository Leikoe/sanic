# borderColor

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.12, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/bordercolor>

The border color for clamped texture values.

## Declaration

```swift
var borderColor: MTLSamplerBorderColor { get set }
```

## Discussion

This value is only used when the sampler address mode is [MTLSamplerAddressMode.clampToBorderColor](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptobordercolor).

## See also

### Declaring addressing modes
- [rAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/raddressmode) — The address mode for the texture depth (r) coordinate.
- [sAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/saddressmode) — The address mode for the texture width (s) coordinate.
- [tAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/taddressmode) — The address mode for the texture height (t) coordinate.
- [MTLSamplerAddressMode](https://developer.apple.com/documentation/metal/mtlsampleraddressmode) — Modes that determine the texture coordinate at each pixel when a fetch falls outside the bounds of a texture.
- [MTLSamplerBorderColor](https://developer.apple.com/documentation/metal/mtlsamplerbordercolor) — Values that determine the border color for clamped texture values when the sampler address mode is [MTLSamplerAddressMode.clampToBorderColor](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptobordercolor).
