# Adding mipmap filtering to samplers

*Article*

<https://developer.apple.com/documentation/metal/adding-mipmap-filtering-to-samplers>

Specify how the GPU samples mipmaps in your textures.

## Overview

By default, samplers sample data only from mipmap `0`. If your texture contains more than one mipmap, and you want it to sample the lower-level mipmaps, you need to specify this behavior when you create the texture sampler.

### Create the sampler in your app

If you’re creating an [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance, create the [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) instance and set its [mipFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/mipfilter) property. The following code uses linear filtering for the minification and magnification filter, and uses linear filtering for mipmaps. This combination is usually called *trilinear filtering*. With this configuration, the GPU chooses the two mipmaps nearest in size and generates a sample by linearly filtering four pixels from each mipmap. Then it blends those two values with a linear interpolation to generate the final sample.

```swift
let descriptor = MTLSamplerDescriptor()
descriptor.minFilter = MTLSamplerMinMagFilter.linear
descriptor.magFilter = MTLSamplerMinMagFilter.linear
descriptor.mipFilter = MTLSamplerMipFilter.linear

let sampler = device.makeSamplerState(descriptor: descriptor)
```

```objective-c
MTLSamplerDescriptor *descriptor = [MTLSamplerDescriptor new];
descriptor.minFilter = MTLSamplerMinMagFilterLinear;
descriptor.magFilter = MTLSamplerMinMagFilterLinear;
descriptor.mipFilter = MTLSamplerMipFilterLinear;

id<MTLSamplerState> sampler = [_device newSamplerStateWithDescriptor: descriptor];
```

Alternatively, any of these filters could filter from the nearest pixel, instead of a linear filter, resulting in fewer sampled pixels but lower quality. Ultimately, you need to decide the right tradeoffs between sampling performance and quality for your app.

### Create the sampler in your shader

If you prefer to create samplers in your shader, specify the mipmap filtering there instead of in your app:

```metal
constexpr sampler s(filter::linear, mip_filter::linear)
```

## See also

### Texture mipmapping
- [Improving texture sampling quality and performance with mipmaps](https://developer.apple.com/documentation/metal/improving-texture-sampling-quality-and-performance-with-mipmaps) — Avoid texture-rendering artifacts and reduce the GPU’s workload by creating smaller versions of a texture.
- [Creating a mipmapped texture](https://developer.apple.com/documentation/metal/creating-a-mipmapped-texture) — Decide whether a texture that you’re creating needs mipmaps.
- [Copying data into or out of mipmaps](https://developer.apple.com/documentation/metal/copying-data-into-or-out-of-mipmaps) — Specify which mipmaps that the data transfer affects.
- [Generating mipmap data](https://developer.apple.com/documentation/metal/generating-mipmap-data) — Create your mipmaps either when you author content or at runtime.
- [Restricting access to specific mipmaps](https://developer.apple.com/documentation/metal/restricting-access-to-specific-mipmaps) — Set the range of mipmap levels that a sampler can access.
- [Predicting which mips the GPU samples with level-of-detail queries](https://developer.apple.com/documentation/metal/predicting-which-mips-the-gpu-samples-with-level-of-detail-queries) — Determine in advance which mipmap levels the GPU requires to sample a texture.
- [Dynamically adjusting texture level of detail](https://developer.apple.com/documentation/metal/dynamically-adjusting-texture-level-of-detail) — Defer generating or loading larger mipmaps until that level of detail is needed.
