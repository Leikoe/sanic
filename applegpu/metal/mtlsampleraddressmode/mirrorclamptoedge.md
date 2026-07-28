# MTLSamplerAddressMode.mirrorClampToEdge

*Case · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.11, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsampleraddressmode/mirrorclamptoedge>

Between `-1.0` and `1.0`, the texture coordinates are mirrored across the axis; outside `-1.0` and `1.0`, texture coordinates are clamped.

## Declaration

```swift
case mirrorClampToEdge
```

## See also

### Address mode options
- [MTLSamplerAddressMode.clampToEdge](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptoedge) — Texture coordinates are clamped between `0.0` and `1.0`, inclusive.
- [MTLSamplerAddressMode.repeat](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/repeat) — Texture coordinates wrap to the other side of the texture, effectively keeping only the fractional part of the texture coordinate.
- [MTLSamplerAddressMode.mirrorRepeat](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/mirrorrepeat) — Between `-1.0` and `1.0`, the texture coordinates are mirrored across the axis; outside `-1.0` and `1.0`, the image is repeated.
- [MTLSamplerAddressMode.clampToZero](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptozero) — Out-of-range texture coordinates return transparent zero `(0,0,0,0)` for images with an alpha channel and return opaque zero `(0,0,0,1)` for images without an alpha channel.
- [MTLSamplerAddressMode.clampToBorderColor](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptobordercolor) — An address mode that returns the sampler’s border color.
