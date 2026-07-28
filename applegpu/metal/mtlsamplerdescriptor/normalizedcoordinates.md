# normalizedCoordinates

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/normalizedcoordinates>

A Boolean value that indicates whether texture coordinates are normalized to the range `[0.0, 1.0]`.

## Declaration

```swift
var normalizedCoordinates: Bool { get set }
```

## Discussion

If [true](https://developer.apple.com/documentation/Swift/true), texture coordinates are from `0.0` to `1.0`. If [false](https://developer.apple.com/documentation/Swift/false), texture coordinates are from `0` to `width` for horizontal coordinates and `0` to `height` for vertical coordinates. The default value is [true](https://developer.apple.com/documentation/Swift/true).

Non-normalized texture coordinates should only be used with 1D and 2D textures with the following conditions; otherwise, the results of sampling are undefined.

- The [MTLSamplerAddressMode.clampToEdge](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptoedge) or [MTLSamplerAddressMode.clampToZero](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptozero) address mode.

- The [MTLSamplerMipFilter.notMipmapped](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/notmipmapped) mipmap filtering option.

- [minFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/minfilter) and [magFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/magfilter) need to be equal to each other.

- [maxAnisotropy](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/maxanisotropy) needs to be `1`.
