# MTLTextureDescriptor

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexturedescriptor>

An instance that you use to configure new Metal texture instances.

## Declaration

```swift
class MTLTextureDescriptor
```

## Overview

To create a new texture, first create an [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) instance and set its property values. Then, call either the [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)) or [makeTexture(descriptor:iosurface:plane:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:iosurface:plane:)) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance, or the [makeTexture(descriptor:offset:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtlbuffer/maketexture(descriptor:offset:bytesperrow:)) method of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance.

When you create a texture, Metal copies property values from the descriptor into the new texture. You can reuse an [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) instance, modifying its property values as needed, to create more [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instances, without affecting any textures you already created.

## Topics

### Creating texture descriptors
- [texture2DDescriptor(pixelFormat:width:height:mipmapped:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texture2ddescriptor(pixelformat:width:height:mipmapped:)) — Creates a texture descriptor object for a 2D texture.
- [textureCubeDescriptor(pixelFormat:size:mipmapped:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturecubedescriptor(pixelformat:size:mipmapped:)) — Creates a texture descriptor object for a cube texture.
- [textureBufferDescriptor(with:width:resourceOptions:usage:)](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturebufferdescriptor(with:width:resourceoptions:usage:)) — Creates a texture descriptor object for a texture buffer.

### Specifying texture attributes
- [textureType](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturetype) — The dimension and arrangement of texture image data.
- [pixelFormat](https://developer.apple.com/documentation/metal/mtltexturedescriptor/pixelformat) — The size and bit layout of all pixels in the texture.
- [width](https://developer.apple.com/documentation/metal/mtltexturedescriptor/width) — The width of the texture image for the base level mipmap, in pixels.
- [height](https://developer.apple.com/documentation/metal/mtltexturedescriptor/height) — The height of the texture image for the base level mipmap, in pixels.
- [depth](https://developer.apple.com/documentation/metal/mtltexturedescriptor/depth) — The depth of the texture image for the base level mipmap, in pixels.
- [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/mipmaplevelcount) — The number of mipmap levels for this texture.
- [sampleCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/samplecount) — The number of samples in each fragment.
- [arrayLength](https://developer.apple.com/documentation/metal/mtltexturedescriptor/arraylength) — The number of array elements for this texture.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtltexturedescriptor/resourceoptions) — The behavior of a new memory allocation.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/cpucachemode) — The CPU cache mode used for the CPU mapping of the texture.
- [storageMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/storagemode) — The location and access permissions of the texture.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/hazardtrackingmode) — The texture’s hazard tracking mode.
- [allowGPUOptimizedContents](https://developer.apple.com/documentation/metal/mtltexturedescriptor/allowgpuoptimizedcontents) — A Boolean value indicating whether the GPU is allowed to adjust the texture’s contents to improve GPU performance.
- [usage](https://developer.apple.com/documentation/metal/mtltexturedescriptor/usage) — Options that determine how you can use the texture.
- [swizzle](https://developer.apple.com/documentation/metal/mtltexturedescriptor/swizzle) — The pattern you want the GPU to apply to pixels when you read or sample pixels from the texture.
- [MTLTextureSwizzleChannels](https://developer.apple.com/documentation/metal/mtltextureswizzlechannels) — A pattern that modifies the data read or sampled from a texture by rearranging or duplicating the elements of a vector.
- [MTLTextureSwizzle](https://developer.apple.com/documentation/metal/mtltextureswizzle) — A set of options to choose from when creating a texture swizzle pattern.
- [MTLTextureType](https://developer.apple.com/documentation/metal/mtltexturetype) — The dimension of each image, including whether multiple images are arranged into an array or a cube.
- [MTLTextureUsage](https://developer.apple.com/documentation/metal/mtltextureusage) — An enumeration for the various options that determine how you can use a texture.

### Instance Properties
- [compressionType](https://developer.apple.com/documentation/metal/mtltexturedescriptor/compressiontype)
- [placementSparsePageSize](https://developer.apple.com/documentation/metal/mtltexturedescriptor/placementsparsepagesize) — Determines the page size for a placement sparse texture.

## See also

### Texture basics
- [Understanding color-renderable pixel format sizes](https://developer.apple.com/documentation/metal/understanding-color-renderable-pixel-format-sizes) — Know the size limits of color render targets in Apple GPUs based on the target’s pixel format.
- [Optimizing texture data](https://developer.apple.com/documentation/metal/optimizing-texture-data) — Optimize a texture’s data to improve GPU or CPU access.
- [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) — A resource that holds formatted image data.
- [MTLTextureCompressionType](https://developer.apple.com/documentation/metal/mtltexturecompressiontype)
- [MTKTextureLoader](https://developer.apple.com/documentation/MetalKit/MTKTextureLoader) — An object that creates textures from existing data in common image formats.
- [MTLSharedTextureHandle](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle) — A texture handle that can be shared across process address space boundaries.
- [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) — The data formats that describe the organization and characteristics of individual pixels in a texture.
