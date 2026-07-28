# MTLSamplerMipFilter

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplermipfilter>

Filtering options for determining what pixel value is returned with multiple mipmap levels.

## Declaration

```swift
enum MTLSamplerMipFilter
```

## Topics

### Specifying mip filter options
- [MTLSamplerMipFilter.notMipmapped](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/notmipmapped) — The texture is sampled from mipmap level `0`, and other mipmap levels are ignored.
- [MTLSamplerMipFilter.nearest](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/nearest) — The nearest mipmap level is selected.
- [MTLSamplerMipFilter.linear](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/linear) — If the filter falls between mipmap levels, both levels are sampled and the results are determined by linear interpolation between levels.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlsamplermipfilter/init(rawvalue:))

## See also

### Declaring filter modes
- [minFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/minfilter) — The filtering option for combining pixels within one mipmap level when the sample footprint is larger than a pixel (minification).
- [magFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/magfilter) — The filtering operation for combining pixels within one mipmap level when the sample footprint is smaller than a pixel (magnification).
- [mipFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/mipfilter) — The filtering option for combining pixels between two mipmap levels.
- [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) — The minimum level of detail (LOD) to use when sampling from a texture.
- [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) — The maximum level of detail (LOD) to use when sampling from a texture.
- [lodAverage](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodaverage) — A Boolean value that specifies whether the GPU can use an average level of detail (LOD) when sampling from a texture.
- [maxAnisotropy](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/maxanisotropy) — The number of samples that can be taken to improve the quality of sample footprints that are anisotropic.
- [MTLSamplerMinMagFilter](https://developer.apple.com/documentation/metal/mtlsamplerminmagfilter) — Filtering options for determining which pixel value is returned within a mipmap level.
