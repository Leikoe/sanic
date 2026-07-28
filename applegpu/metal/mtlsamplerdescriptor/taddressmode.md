# tAddressMode

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/taddressmode>

The address mode for the texture height (t) coordinate.

## Declaration

```swift
var tAddressMode: MTLSamplerAddressMode { get set }
```

## Discussion

The default value is [MTLSamplerAddressMode.clampToEdge](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptoedge).

## See also

### Declaring addressing modes
- [rAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/raddressmode) — The address mode for the texture depth (r) coordinate.
- [sAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/saddressmode) — The address mode for the texture width (s) coordinate.
- [borderColor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/bordercolor) — The border color for clamped texture values.
- [MTLSamplerAddressMode](https://developer.apple.com/documentation/metal/mtlsampleraddressmode) — Modes that determine the texture coordinate at each pixel when a fetch falls outside the bounds of a texture.
- [MTLSamplerBorderColor](https://developer.apple.com/documentation/metal/mtlsamplerbordercolor) — Values that determine the border color for clamped texture values when the sampler address mode is [MTLSamplerAddressMode.clampToBorderColor](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptobordercolor).
