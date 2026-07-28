# MTLSamplerMipFilter.linear

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplermipfilter/linear>

If the filter falls between mipmap levels, both levels are sampled and the results are determined by linear interpolation between levels.

## Declaration

```swift
case linear
```

## Discussion

Support for linear filtering between mipmaps varies by GPU and the format of the texture being sampled. For example, you can’t use linear filtering on textures with an integer format, and only some device objects support linear filtering for textures with a floating-point format. To determine whether linear filtering is available for a specific texture format, see:

- [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf)

- [Metal feature set tables (Numbers)](https://developer.apple.com/metal/metal-feature-set-tables.zip)

## See also

### Specifying mip filter options
- [MTLSamplerMipFilter.notMipmapped](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/notmipmapped) — The texture is sampled from mipmap level `0`, and other mipmap levels are ignored.
- [MTLSamplerMipFilter.nearest](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/nearest) — The nearest mipmap level is selected.
