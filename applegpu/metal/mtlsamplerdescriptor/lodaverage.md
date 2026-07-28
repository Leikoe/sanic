# lodAverage

*Instance Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 14.0, macOS 11.0, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodaverage>

A Boolean value that specifies whether the GPU can use an average level of detail (LOD) when sampling from a texture.

## Declaration

```swift
var lodAverage: Bool { get set }
```

## Discussion

If this value is [true](https://developer.apple.com/documentation/Swift/true), an average LOD may be used across four fragment shader threads. If this value is [false](https://developer.apple.com/documentation/Swift/false), no averaging is performed and each thread accesses its own LOD.

The default value is [false](https://developer.apple.com/documentation/Swift/false).

> **Note:**
>  This optional Boolean value is used as a performance optimization hint and it is ignored on some GPUs. Enabling LOD averaging may provide a performance benefit for shaders that sample from explicit per-fragment mipmap levels, or apply per-fragment LOD bias, at the potential cost of reduced texture sample quality.

## See also

### Declaring filter modes
- [minFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/minfilter) — The filtering option for combining pixels within one mipmap level when the sample footprint is larger than a pixel (minification).
- [magFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/magfilter) — The filtering operation for combining pixels within one mipmap level when the sample footprint is smaller than a pixel (magnification).
- [mipFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/mipfilter) — The filtering option for combining pixels between two mipmap levels.
- [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) — The minimum level of detail (LOD) to use when sampling from a texture.
- [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) — The maximum level of detail (LOD) to use when sampling from a texture.
- [maxAnisotropy](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/maxanisotropy) — The number of samples that can be taken to improve the quality of sample footprints that are anisotropic.
- [MTLSamplerMinMagFilter](https://developer.apple.com/documentation/metal/mtlsamplerminmagfilter) — Filtering options for determining which pixel value is returned within a mipmap level.
- [MTLSamplerMipFilter](https://developer.apple.com/documentation/metal/mtlsamplermipfilter) — Filtering options for determining what pixel value is returned with multiple mipmap levels.
