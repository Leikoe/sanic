# MTLSharedTextureHandle

*Class · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.14, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsharedtexturehandle>

A texture handle that can be shared across process address space boundaries.

## Declaration

```swift
class MTLSharedTextureHandle
```

## Overview

`MTLSharedTextureHandle` objects may be passed between processes using XPC connections and then used to create a reference to the texture in another process. The texture in the other process needs to be created using the same [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) on which the shared texture was originally created. To identify which device it was created on, you can use the [device](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle/device) property of the `MTLSharedTextureHandle` object.

## Topics

### Identifying the shared texture handle
- [device](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle/device) — The device object that created the texture.
- [label](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle/label) — A string that identifies the texture.

### Initializers
- [init(coder:)](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle/init(coder:))

## See also

### Texture basics
- [Understanding color-renderable pixel format sizes](https://developer.apple.com/documentation/metal/understanding-color-renderable-pixel-format-sizes) — Know the size limits of color render targets in Apple GPUs based on the target’s pixel format.
- [Optimizing texture data](https://developer.apple.com/documentation/metal/optimizing-texture-data) — Optimize a texture’s data to improve GPU or CPU access.
- [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) — A resource that holds formatted image data.
- [MTLTextureCompressionType](https://developer.apple.com/documentation/metal/mtltexturecompressiontype)
- [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) — An instance that you use to configure new Metal texture instances.
- [MTKTextureLoader](https://developer.apple.com/documentation/MetalKit/MTKTextureLoader) — An object that creates textures from existing data in common image formats.
- [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) — The data formats that describe the organization and characteristics of individual pixels in a texture.
