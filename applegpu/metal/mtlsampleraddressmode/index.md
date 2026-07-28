# MTLSamplerAddressMode

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsampleraddressmode>

Modes that determine the texture coordinate at each pixel when a fetch falls outside the bounds of a texture.

## Declaration

```swift
enum MTLSamplerAddressMode
```

## Topics

### Address mode options
- [MTLSamplerAddressMode.clampToEdge](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptoedge) — Texture coordinates are clamped between `0.0` and `1.0`, inclusive.
- [MTLSamplerAddressMode.mirrorClampToEdge](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/mirrorclamptoedge) — Between `-1.0` and `1.0`, the texture coordinates are mirrored across the axis; outside `-1.0` and `1.0`, texture coordinates are clamped.
- [MTLSamplerAddressMode.repeat](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/repeat) — Texture coordinates wrap to the other side of the texture, effectively keeping only the fractional part of the texture coordinate.
- [MTLSamplerAddressMode.mirrorRepeat](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/mirrorrepeat) — Between `-1.0` and `1.0`, the texture coordinates are mirrored across the axis; outside `-1.0` and `1.0`, the image is repeated.
- [MTLSamplerAddressMode.clampToZero](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptozero) — Out-of-range texture coordinates return transparent zero `(0,0,0,0)` for images with an alpha channel and return opaque zero `(0,0,0,1)` for images without an alpha channel.
- [MTLSamplerAddressMode.clampToBorderColor](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptobordercolor) — An address mode that returns the sampler’s border color.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/init(rawvalue:))

## See also

### Declaring addressing modes
- [rAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/raddressmode) — The address mode for the texture depth (r) coordinate.
- [sAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/saddressmode) — The address mode for the texture width (s) coordinate.
- [tAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/taddressmode) — The address mode for the texture height (t) coordinate.
- [borderColor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/bordercolor) — The border color for clamped texture values.
- [MTLSamplerBorderColor](https://developer.apple.com/documentation/metal/mtlsamplerbordercolor) — Values that determine the border color for clamped texture values when the sampler address mode is [MTLSamplerAddressMode.clampToBorderColor](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptobordercolor).
