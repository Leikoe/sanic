# reductionMode

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/reductionmode>

Sets the reduction mode for filtering contributing samples.

## Declaration

```swift
var reductionMode: MTLSamplerReductionMode { get set }
```

## Discussion

The property’s default value is `MTLSamplerReductionModeWeightedAverage`. The sampler ignores this property if any of the following property values are equal to a specific value:

- The sampler’s [mipFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/mipfilter) property is equal to `MTLSamplerMipFilterNotMipmapped`.

- The sampler’s [mipFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/mipfilter) property is equal to `MTLSamplerMipFilterNearest`.

- The sampler’s [minFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/minfilter) property is equal to `MTLSamplerMinMagFilterNearest`.

- The sampler’s [magFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/magfilter) property is equal to `MTLSamplerMinMagFilterNearest`.
