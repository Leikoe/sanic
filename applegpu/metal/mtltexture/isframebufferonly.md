# isFramebufferOnly

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/isframebufferonly>

A Boolean value that indicates whether the texture can only be used as a render target.

## Declaration

```swift
var isFramebufferOnly: Bool { get }
```

## Discussion

The default is [false](https://developer.apple.com/documentation/Swift/false), which indicates the use of the texture is not restricted.

If [true](https://developer.apple.com/documentation/Swift/true), neither [replace(region:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:slice:withbytes:bytesperrow:bytesperimage:)) nor [getBytes(_:bytesPerRow:bytesPerImage:from:mipmapLevel:slice:)](https://developer.apple.com/documentation/metal/mtltexture/getbytes(_:bytesperrow:bytesperimage:from:mipmaplevel:slice:)) can be used with this texture. Also, this texture can only be used as an attachment for [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) and cannot be a texture argument for [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder), [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder), or [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder).

Textures you obtain from a [CAMetalDrawable](https://developer.apple.com/documentation/QuartzCore/CAMetalDrawable) instance are only usable as attachments, depending on the value of [framebufferOnly](https://developer.apple.com/documentation/QuartzCore/CAMetalLayer/framebufferOnly) passed to their parent [CAMetalLayer](https://developer.apple.com/documentation/QuartzCore/CAMetalLayer) instance. These restrictions don’t apply to textures that your app creates directly.

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
- [usage](https://developer.apple.com/documentation/metal/mtltexture/usage) — Options that determine how you can use the texture.
- [allowGPUOptimizedContents](https://developer.apple.com/documentation/metal/mtltexture/allowgpuoptimizedcontents) — A Boolean value indicating whether the GPU is allowed to adjust the contents of the texture to improve GPU performance.
- [isShareable](https://developer.apple.com/documentation/metal/mtltexture/isshareable) — A Boolean indicating whether this texture can be shared with other processes.
- [swizzle](https://developer.apple.com/documentation/metal/mtltexture/swizzle) — The pattern that the GPU applies to pixels when you read or sample pixels from the texture.
- [MTLTextureType](https://developer.apple.com/documentation/metal/mtltexturetype) — The dimension of each image, including whether multiple images are arranged into an array or a cube.
- [MTLTextureUsage](https://developer.apple.com/documentation/metal/mtltextureusage) — An enumeration for the various options that determine how you can use a texture.
