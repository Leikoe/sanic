# Restricting access to specific mipmaps

*Article*

<https://developer.apple.com/documentation/metal/restricting-access-to-specific-mipmaps>

Set the range of mipmap levels that a sampler can access.

## Overview

Sometimes, you want to control the specific mipmap levels that the sampler can read from. For example, you might do this when you haven’t provided texture data for all of the mipmaps, and you want to constrain access to the mipmaps that have data. You can configure a sampler to read from a subset of the texture’s mipmaps.

### Limit the sampler when you create it in your Metal app

When you configure the [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) instance, set the [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) and [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) properties to the range of permitted values.

```swift
let descriptor = MTLSamplerDescriptor()
descriptor.minFilter = MTLSamplerMinMagFilter.linear
descriptor.magFilter = MTLSamplerMinMagFilter.linear
descriptor.mipFilter = MTLSamplerMipFilter.linear

descriptor.lodMinClamp = 3.0
descriptor.lodMaxClamp = 5.0

let sampler = device.makeSamplerState(descriptor: descriptor)
```

```objective-c
MTLSamplerDescriptor *descriptor = [MTLSamplerDescriptor new];
descriptor.minFilter = MTLSamplerMinMagFilterLinear;
descriptor.magFilter = MTLSamplerMinMagFilterLinear;
descriptor.mipFilter = MTLSamplerMipFilterLinear;

descriptor.lodMinClamp = 3.0f;
descriptor.lodMaxClamp = 5.0f;

id<MTLSamplerState> sampler = [device newSamplerStateWithDescriptor: descriptor];
```

This example creates a sampler that ignores mipmaps `0`, `1`, and `2`.

### Limit the sampler when you create it in your shader

If you create your sampler in your shader, specify the range of mipmap levels that it can access:

```metal
constexpr sampler s(filter::linear, mip_filter::linear, lod_clamp(3.0f, MAXFLOAT))
```

### Control mipmap selection when you sample the texture

Some GPUs can apply additional constraints on the sample operation itself, passing in dynamic information about which mipmap levels the GPU can sample.

Not all GPUs support clamping at the moment it samples a texture. Verify that GPU’s device instance supports clamping to a minimum level-of-detail (LOD) by checking whether it supports one of the following:

- The [MTLGPUFamily.mac2](https://developer.apple.com/documentation/metal/mtlgpufamily/mac2) feature set.

- The [MTLGPUFamily.apple6](https://developer.apple.com/documentation/metal/mtlgpufamily/apple6) feature set.

```swift
let macFamily2Support = device.supportsFamily(MTLGPUFamily.mac2)
let appleFamily6Support = device.supportsFamily(MTLGPUFamily.apple6)
let supportsMinLevelOfDetailClamp = macFamily2Support || appleFamily6Support
```

```objective-c
Boolean macFamily2Support = [device supportsFamily:MTLGPUFamilyMac2];
Boolean appleFamily6Support = [device supportsFamily:MTLGPUFamilyApple6];
Boolean supportsMinLevelOfDetailClamp = macFamily2Support || appleFamily6Support;
```

In your shader, call one of the variants of the `sample` function that takes additional LOD parameters. For example, the following code limits sampling to a specific level or lower in the mipmap chain. The shader has a minimum level parameter that it uses to sample the texture:

```metal
fragment float4
samplingShader(RasterizerData in [[stage_in]],
               texture2d<half> colorTexture [[ texture(0) ]],
               constant float &minimumLOD [[buffer(0)]])
{
    constexpr sampler textureSampler (mag_filter::linear,
                                      min_filter::linear,
                                      mip_filter::linear);

    const half4 colorSample = colorTexture.sample(textureSampler,
                                                  in.textureCoordinate,
                                                  min_lod_clamp(minimumLOD));
    
    return float4(colorSample);
}
```

This example limits sampling to a specific level or lower in the mipmap chain. The shader has a minimum level parameter, `minimumLOD`, that it uses to sample the texture.

The [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364) describes other options for controlling mipmap selection. You can choose to sample a specific mipmap level, specify a minimum mipmap level, bias the selection process that the hardware chooses, or use some combination of these options.

## See also

### Texture mipmapping
- [Improving texture sampling quality and performance with mipmaps](https://developer.apple.com/documentation/metal/improving-texture-sampling-quality-and-performance-with-mipmaps) — Avoid texture-rendering artifacts and reduce the GPU’s workload by creating smaller versions of a texture.
- [Creating a mipmapped texture](https://developer.apple.com/documentation/metal/creating-a-mipmapped-texture) — Decide whether a texture that you’re creating needs mipmaps.
- [Copying data into or out of mipmaps](https://developer.apple.com/documentation/metal/copying-data-into-or-out-of-mipmaps) — Specify which mipmaps that the data transfer affects.
- [Generating mipmap data](https://developer.apple.com/documentation/metal/generating-mipmap-data) — Create your mipmaps either when you author content or at runtime.
- [Adding mipmap filtering to samplers](https://developer.apple.com/documentation/metal/adding-mipmap-filtering-to-samplers) — Specify how the GPU samples mipmaps in your textures.
- [Predicting which mips the GPU samples with level-of-detail queries](https://developer.apple.com/documentation/metal/predicting-which-mips-the-gpu-samples-with-level-of-detail-queries) — Determine in advance which mipmap levels the GPU requires to sample a texture.
- [Dynamically adjusting texture level of detail](https://developer.apple.com/documentation/metal/dynamically-adjusting-texture-level-of-detail) — Defer generating or loading larger mipmaps until that level of detail is needed.
