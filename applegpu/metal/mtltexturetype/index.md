# MTLTextureType

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexturetype>

The dimension of each image, including whether multiple images are arranged into an array or a cube.

## Declaration

```swift
enum MTLTextureType
```

## Overview

For a `MTLTextureTypeCube` texture, the property values describe one slice, which is any one of its six sides. For example, [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexture/mipmaplevelcount) is the number of mipmap levels for one slice, not the total sum of mipmap levels in six slices. By definition, the [width](https://developer.apple.com/documentation/metal/mtltexture/width) and [height](https://developer.apple.com/documentation/metal/mtltexture/height) of a cube texture are the same value.

Each slice of a cube texture maps to a side with a specific orientation.

| Slice index | Slice orientation |
|---|---|
| 0 | +X |
| 1 | -X |
| 2 | +Y |
| 3 | -Y |
| 4 | +Z |
| 5 | -Z |

## Topics

### Specifying the texture type
- [MTLTextureType.type1D](https://developer.apple.com/documentation/metal/mtltexturetype/type1d) — A one-dimensional texture image.
- [MTLTextureType.type1DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type1darray) — An array of one-dimensional texture images.
- [MTLTextureType.type2D](https://developer.apple.com/documentation/metal/mtltexturetype/type2d) — A two-dimensional texture image.
- [MTLTextureType.type2DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2darray) — An array of two-dimensional texture images.
- [MTLTextureType.type2DMultisample](https://developer.apple.com/documentation/metal/mtltexturetype/type2dmultisample) — A two-dimensional texture image that uses more than one sample for each pixel.
- [MTLTextureType.typeCube](https://developer.apple.com/documentation/metal/mtltexturetype/typecube) — A cube texture with six two-dimensional images.
- [MTLTextureType.typeCubeArray](https://developer.apple.com/documentation/metal/mtltexturetype/typecubearray) — An array of cube textures, each with six two-dimensional images.
- [MTLTextureType.type3D](https://developer.apple.com/documentation/metal/mtltexturetype/type3d) — A three-dimensional texture image.
- [MTLTextureType.type2DMultisampleArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2dmultisamplearray) — An array of two-dimensional texture images that use more than one sample for each pixel.
- [MTLTextureType.typeTextureBuffer](https://developer.apple.com/documentation/metal/mtltexturetype/typetexturebuffer) — A texture buffer.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtltexturetype/init(rawvalue:))

## See also

### Querying texture attributes
- [textureType](https://developer.apple.com/documentation/metal/mtltexture/texturetype) — The dimension and arrangement of the texture image data.
- [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) — The format of pixels in the texture.
- [width](https://developer.apple.com/documentation/metal/mtltexture/width) — The width of the texture image for the base level mipmap, in pixels.
- [height](https://developer.apple.com/documentation/metal/mtltexture/height) — The height of the texture image for the base level mipmap, in pixels.
- [depth](https://developer.apple.com/documentation/metal/mtltexture/depth) — The depth of the texture image for the base level mipmap, in pixels.
- [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexture/mipmaplevelcount) — The number of mipmap levels in the texture.
- [arrayLength](https://developer.apple.com/documentation/metal/mtltexture/arraylength) — The number of slices in the texture array.
- [sampleCount](https://developer.apple.com/documentation/metal/mtltexture/samplecount) — The number of samples in each pixel.
- [isFramebufferOnly](https://developer.apple.com/documentation/metal/mtltexture/isframebufferonly) — A Boolean value that indicates whether the texture can only be used as a render target.
- [usage](https://developer.apple.com/documentation/metal/mtltexture/usage) — Options that determine how you can use the texture.
- [allowGPUOptimizedContents](https://developer.apple.com/documentation/metal/mtltexture/allowgpuoptimizedcontents) — A Boolean value indicating whether the GPU is allowed to adjust the contents of the texture to improve GPU performance.
- [isShareable](https://developer.apple.com/documentation/metal/mtltexture/isshareable) — A Boolean indicating whether this texture can be shared with other processes.
- [swizzle](https://developer.apple.com/documentation/metal/mtltexture/swizzle) — The pattern that the GPU applies to pixels when you read or sample pixels from the texture.
- [MTLTextureUsage](https://developer.apple.com/documentation/metal/mtltextureusage) — An enumeration for the various options that determine how you can use a texture.
