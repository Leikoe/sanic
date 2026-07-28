# MTLTexture

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture>

A resource that holds formatted image data.

## Declaration

```swift
protocol MTLTexture : MTLResource
```

## Overview

Don’t implement this protocol yourself; instead, use one of the following methods to create an [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance:

- Create an [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) instance to describe the texture’s properties and then call the [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)) method of the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) protocol to create the texture.

- To create a texture that uses an existing [IOSurface](https://developer.apple.com/documentation/IOSurface/IOSurface) to hold the texture data, create an [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) instance to describe the image data in the surface. Call the [makeTexture(descriptor:iosurface:plane:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:iosurface:plane:)) method to create the texture.

- To create a texture that reinterprets another texture’s data as if it has a different format, call one of the following texture methods:

  - [makeTextureView(pixelFormat:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:))

  - [makeTextureView(pixelFormat:textureType:levels:slices:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:texturetype:levels:slices:)) (Swift)

  - [newTextureViewWithPixelFormat:textureType:levels:slices:](https://developer.apple.com/documentation/metal/mtltexture/newtextureviewwithpixelformat:texturetype:levels:slices:) (Objective-C)

  You need to choose a pixel format for the new texture compatible with the source texture’s pixel format. The new texture shares the same storage allocation as the source texture. If you make changes to the new texture, the source texture reflects those changes, and vice versa.

- To create a texture that uses an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance’s contents to hold pixel data, create an [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) instance to describe the texture’s properties. Then call the [makeTexture(descriptor:offset:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtlbuffer/maketexture(descriptor:offset:bytesperrow:)) method on the buffer instance. The new texture instance shares the storage allocation of the source buffer instance. If you make changes to the texture, the buffer reflects those changes, and vice versa.

After you create an [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance, most of its characteristics, such as its size, type, and pixel format are all immutable. Only the texture’s pixel data is mutable.

To copy pixel data from system memory into the texture, call [replace(region:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:slice:withbytes:bytesperrow:bytesperimage:)) or [replace(region:mipmapLevel:withBytes:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:withbytes:bytesperrow:)).

To copy pixel data back to system memory, call [getBytes(_:bytesPerRow:bytesPerImage:from:mipmapLevel:slice:)](https://developer.apple.com/documentation/metal/mtltexture/getbytes(_:bytesperrow:bytesperimage:from:mipmaplevel:slice:)) or [getBytes(_:bytesPerRow:from:mipmapLevel:)](https://developer.apple.com/documentation/metal/mtltexture/getbytes(_:bytesperrow:from:mipmaplevel:)).

## Topics

### Copying data into a texture image
- [replace(region:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:slice:withbytes:bytesperrow:bytesperimage:)) — Copies pixel data into a section of a texture slice.
- [replace(region:mipmapLevel:withBytes:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:withbytes:bytesperrow:)) — Copies a block of pixels into a section of texture slice 0.

### Copying data from a texture image
- [getBytes(_:bytesPerRow:bytesPerImage:from:mipmapLevel:slice:)](https://developer.apple.com/documentation/metal/mtltexture/getbytes(_:bytesperrow:bytesperimage:from:mipmaplevel:slice:)) — Copies pixel data from the texture to a buffer in system memory.
- [getBytes(_:bytesPerRow:from:mipmapLevel:)](https://developer.apple.com/documentation/metal/mtltexture/getbytes(_:bytesperrow:from:mipmaplevel:)) — Copies pixel data from the first slice of the texture to a buffer in system memory.

### Creating textures by reinterpreting existing texture data
- [makeTextureView(pixelFormat:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:)) — Creates a new view of the texture, reinterpreting its data using a different pixel format.
- [makeTextureView(pixelFormat:textureType:levels:slices:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:texturetype:levels:slices:)) — Creates a new view of the texture, reinterpreting a subset of its data using a different type and pixel format.
- [makeTextureView(pixelFormat:textureType:levels:slices:swizzle:)](https://developer.apple.com/documentation/metal/mtltexture/maketextureview(pixelformat:texturetype:levels:slices:swizzle:)) — Creates a new view of the texture, reinterpreting a subset of its data using a different type, pixel format, and swizzle pattern.

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
- [MTLTextureUsage](https://developer.apple.com/documentation/metal/mtltextureusage) — An enumeration for the various options that determine how you can use a texture.

### Getting information about the IOSurface the texture was created from
- [iosurface](https://developer.apple.com/documentation/metal/mtltexture/iosurface) — A reference to the underlying surface instance for the texture, if applicable.
- [iosurfacePlane](https://developer.apple.com/documentation/metal/mtltexture/iosurfaceplane) — The number of a plane within the underlying surface instance for the texture, if applicable.

### Getting information about ancestor resources
- [parent](https://developer.apple.com/documentation/metal/mtltexture/parent) — The parent texture used to create this texture, if any.
- [parentRelativeLevel](https://developer.apple.com/documentation/metal/mtltexture/parentrelativelevel) — The base level of the parent texture used to create this texture.
- [parentRelativeSlice](https://developer.apple.com/documentation/metal/mtltexture/parentrelativeslice) — The base slice of the parent texture used to create this texture.
- [buffer](https://developer.apple.com/documentation/metal/mtltexture/buffer) — The source buffer used to create this texture, if any.
- [bufferOffset](https://developer.apple.com/documentation/metal/mtltexture/bufferoffset) — The offset in the source buffer where the texture’s data comes from.
- [bufferBytesPerRow](https://developer.apple.com/documentation/metal/mtltexture/bufferbytesperrow) — The number of bytes in each row of the texture’s source buffer.
- [rootResource](https://developer.apple.com/documentation/metal/mtltexture/rootresource) — The resource that owns the storage for this texture.

### Creating a shared texture handle
- [makeSharedTextureHandle()](https://developer.apple.com/documentation/metal/mtltexture/makesharedtexturehandle()) — Creates a new texture handle from a shareable texture.

### Creating views of textures on other GPUs
- [makeRemoteTextureView(_:)](https://developer.apple.com/documentation/metal/mtltexture/makeremotetextureview(_:)) — Creates a remote texture view for another GPU in the same peer group.
- [remoteStorageTexture](https://developer.apple.com/documentation/metal/mtltexture/remotestoragetexture) — The texture on another GPU that the texture was created from, if any.

### Querying sparse properties
- [isSparse](https://developer.apple.com/documentation/metal/mtltexture/issparse) — A Boolean value that indicates whether this is a sparse texture.
- [firstMipmapInTail](https://developer.apple.com/documentation/metal/mtltexture/firstmipmapintail) — The index of the first mipmap in the tail.
- [tailSizeInBytes](https://developer.apple.com/documentation/metal/mtltexture/tailsizeinbytes) — The size of the sparse texture tail, in bytes.

### Instance Properties
- [compressionType](https://developer.apple.com/documentation/metal/mtltexture/compressiontype)
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtltexture/gpuresourceid)
- [sparseTextureTier](https://developer.apple.com/documentation/metal/mtltexture/sparsetexturetier)

### Instance Methods
- [newTextureView(with:)](https://developer.apple.com/documentation/metal/mtltexture/newtextureview(with:))

## See also

### Texture basics
- [Understanding color-renderable pixel format sizes](https://developer.apple.com/documentation/metal/understanding-color-renderable-pixel-format-sizes) — Know the size limits of color render targets in Apple GPUs based on the target’s pixel format.
- [Optimizing texture data](https://developer.apple.com/documentation/metal/optimizing-texture-data) — Optimize a texture’s data to improve GPU or CPU access.
- [MTLTextureCompressionType](https://developer.apple.com/documentation/metal/mtltexturecompressiontype)
- [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) — An instance that you use to configure new Metal texture instances.
- [MTKTextureLoader](https://developer.apple.com/documentation/MetalKit/MTKTextureLoader) — An object that creates textures from existing data in common image formats.
- [MTLSharedTextureHandle](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle) — A texture handle that can be shared across process address space boundaries.
- [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) — The data formats that describe the organization and characteristics of individual pixels in a texture.
