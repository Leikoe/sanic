# MTLSamplerMipFilter.nearest

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplermipfilter/nearest>

The nearest mipmap level is selected.

## Declaration

```swift
case nearest
```

## See also

### Specifying mip filter options
- [MTLSamplerMipFilter.notMipmapped](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/notmipmapped) — The texture is sampled from mipmap level `0`, and other mipmap levels are ignored.
- [MTLSamplerMipFilter.linear](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/linear) — If the filter falls between mipmap levels, both levels are sampled and the results are determined by linear interpolation between levels.
