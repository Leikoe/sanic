# MTLSamplerDescriptor

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplerdescriptor>

An object that you use to configure a texture sampler.

## Declaration

```swift
class MTLSamplerDescriptor
```

## Overview

To make a sampler, create and configure an [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) instance and then call an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeSamplerState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesamplerstate(descriptor:)) method. After you create the sampler, you can release the descriptor or reconfigure its properties to create other samplers.

## Topics

### Declaring the coordinate space
- [normalizedCoordinates](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/normalizedcoordinates) — A Boolean value that indicates whether texture coordinates are normalized to the range `[0.0, 1.0]`.

### Declaring addressing modes
- [rAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/raddressmode) — The address mode for the texture depth (r) coordinate.
- [sAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/saddressmode) — The address mode for the texture width (s) coordinate.
- [tAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/taddressmode) — The address mode for the texture height (t) coordinate.
- [borderColor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/bordercolor) — The border color for clamped texture values.
- [MTLSamplerAddressMode](https://developer.apple.com/documentation/metal/mtlsampleraddressmode) — Modes that determine the texture coordinate at each pixel when a fetch falls outside the bounds of a texture.
- [MTLSamplerBorderColor](https://developer.apple.com/documentation/metal/mtlsamplerbordercolor) — Values that determine the border color for clamped texture values when the sampler address mode is [MTLSamplerAddressMode.clampToBorderColor](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptobordercolor).

### Declaring filter modes
- [minFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/minfilter) — The filtering option for combining pixels within one mipmap level when the sample footprint is larger than a pixel (minification).
- [magFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/magfilter) — The filtering operation for combining pixels within one mipmap level when the sample footprint is smaller than a pixel (magnification).
- [mipFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/mipfilter) — The filtering option for combining pixels between two mipmap levels.
- [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) — The minimum level of detail (LOD) to use when sampling from a texture.
- [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) — The maximum level of detail (LOD) to use when sampling from a texture.
- [lodAverage](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodaverage) — A Boolean value that specifies whether the GPU can use an average level of detail (LOD) when sampling from a texture.
- [maxAnisotropy](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/maxanisotropy) — The number of samples that can be taken to improve the quality of sample footprints that are anisotropic.
- [MTLSamplerMinMagFilter](https://developer.apple.com/documentation/metal/mtlsamplerminmagfilter) — Filtering options for determining which pixel value is returned within a mipmap level.
- [MTLSamplerMipFilter](https://developer.apple.com/documentation/metal/mtlsamplermipfilter) — Filtering options for determining what pixel value is returned with multiple mipmap levels.

### Declaring the depth comparison mode
- [compareFunction](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/comparefunction) — The sampler comparison function used when performing a sample compare operation on a depth texture.
- [MTLCompareFunction](https://developer.apple.com/documentation/metal/mtlcomparefunction) — Options used to specify how a sample compare operation should be performed on a depth texture.

### Declaring whether the sampler can be used in argument buffers
- [supportArgumentBuffers](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/supportargumentbuffers) — A Boolean value that indicates whether you can reference a sampler, that you make with this descriptor, by its resource ID from an argument buffer.

### Identifying the sampler
- [label](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/label) — A string that identifies the sampler.

### Instance Properties
- [lodBias](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodbias) — Sets the level-of-detail (lod) bias when sampling from a texture.
- [reductionMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/reductionmode) — Sets the reduction mode for filtering contributing samples.

## See also

### Texture samplers
- [Creating and sampling textures](https://developer.apple.com/documentation/metal/creating-and-sampling-textures) — Load image data into a texture and apply it to a quadrangle.
- [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) — An instance that defines how a texture should be sampled.
- [MTLSamplePosition](https://developer.apple.com/documentation/metal/mtlsampleposition) — A subpixel sample position for use in multisample antialiasing (MSAA).
- [MTLSamplerReductionMode](https://developer.apple.com/documentation/metal/mtlsamplerreductionmode) — Configures how the sampler aggregates contributing samples to a final value.
