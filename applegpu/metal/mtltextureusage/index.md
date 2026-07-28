# MTLTextureUsage

*Structure · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltextureusage>

An enumeration for the various options that determine how you can use a texture.

## Declaration

```swift
struct MTLTextureUsage
```

## Overview

If a texture has multiple uses in your app, you can combine multiple usage options for that texture. After you set the texture’s usage options, you can use it only in the ways that you specified.

Metal can optimize operations for a given texture, based on its intended use. Set explicit usage options for a texture, if you know them in advance, before you use the texture. Only set usage options that correspond to a texture’s intended use.

In iOS devices with GPU family 5, Metal doesn’t apply lossless compression to a given texture if you set any of these options:

- [unknown](https://developer.apple.com/documentation/metal/mtltextureusage/unknown)

- [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite)

- [pixelFormatView](https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview)

## Topics

### Specifying texture usage options
- [unknown](https://developer.apple.com/documentation/metal/mtltextureusage/unknown) — An option for a texture whose usage is unknown.
- [shaderRead](https://developer.apple.com/documentation/metal/mtltextureusage/shaderread) — An option for reading or sampling from the texture in a shader.
- [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) — An option for writing to the texture in a shader.
- [shaderAtomic](https://developer.apple.com/documentation/metal/mtltextureusage/shaderatomic) — An option that enables atomic memory operations on texture elements in shader code.
- [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) — An option for rendering to the texture in a render pass.
- [pixelFormatView](https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview) — An option to create texture views with a different component layout.

### Creating texture usage options
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtltextureusage/init(rawvalue:)) — Creates new, empty usage options.

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
- [MTLTextureType](https://developer.apple.com/documentation/metal/mtltexturetype) — The dimension of each image, including whether multiple images are arranged into an array or a cube.
