# makeTexture(descriptor:offset:bytesPerRow:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.13, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbuffer/maketexture(descriptor:offset:bytesperrow:)>

Creates a texture that shares its storage with the buffer.

## Declaration

```swift
func makeTexture(descriptor: MTLTextureDescriptor, offset: Int, bytesPerRow: Int) -> (any MTLTexture)?
```

## Parameters

- **descriptor** — The descriptor that contains the properties of the texture.
- **offset** — The offset, in bytes, from the base address for the first row of texture data.
- **bytesPerRow** — The stride, in bytes, from one row of texture data to the next.

## Return Value

A new texture that shares the buffer’s underlying storage.

## Discussion

This method creates a new [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance that uses the same data as the buffer’s. Modifying the buffer also modifies the new texture because they share the same underlying memory.

> **Note:**
>  Metal may not be able to optimize a texture that shares memory with a buffer.

The texture’s resource data is coherent between multiple render passes. However, that data may not be coherent within a single render pass due to caching at runtime. For example, a texture you create from the method may not be able to immediately reflect changes to the underlying buffer that come from a render or kernel function.

If this buffer’s [storageMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/storagemode) is [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed), and a render or kernel function modifies it, the CPU can access the new values through a texture after calling the [synchronize(resource:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(resource:)) method. CPU memory operations are only coherent between command buffer boundaries. GPU barriers guard its memory operations to buffers and textures so that each operation finishes running before the next one begins.

You can create multiple, nonoverlapping textures that use the same buffer; however, the GPU serializes memory operations to those textures.

> **Tip:**
>  You can avoid the GPU’s texture access serialization by creating multiple buffers and then creating a texture from each buffer with this method.

To create a linear texture, you need to:

- Align the `offset` and `bytesPerRow` parameters to the value that the [minimumLinearTextureAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumlineartexturealignment(for:)) method returns.

- Set the `bytesPerRow` parameter to a value greater than or equal to the number of bytes in one row of pixels — the product of the row’s width, in pixels, and the size of one pixel, in bytes.

Additionally, creating a linear texture from this method adds the following restrictions for the `descriptor` parameter’s properties:

| Property | Acceptable values for a linear texture |
|---|---|
| [textureType](https://developer.apple.com/documentation/metal/mtltexturedescriptor/texturetype) | [MTLTextureType.type2D](https://developer.apple.com/documentation/metal/mtltexturetype/type2d) or [MTLTextureType.typeTextureBuffer](https://developer.apple.com/documentation/metal/mtltexturetype/typetexturebuffer) |
| [depth](https://developer.apple.com/documentation/metal/mtltexturedescriptor/depth) | `1` |
| [arrayLength](https://developer.apple.com/documentation/metal/mtltexturedescriptor/arraylength) | `1` |
| [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/mipmaplevelcount) | `1` |
| [sampleCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/samplecount) | `1` |
| [usage](https://developer.apple.com/documentation/metal/mtltexturedescriptor/usage) | The [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) value if the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance supports [MTLGPUFamily.apple1](https://developer.apple.com/documentation/metal/mtlgpufamily/apple1) (see [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:))), or any other [MTLTextureUsage](https://developer.apple.com/documentation/metal/mtltextureusage) value |
| [storageMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/storagemode) | The same value as this buffer’s [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) property (see [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals)) |
| [pixelFormat](https://developer.apple.com/documentation/metal/mtltexturedescriptor/pixelformat) | Any ordinary or packed color [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat), except [MTLPixelFormat.gbgr422](https://developer.apple.com/documentation/metal/mtlpixelformat/gbgr422) and [MTLPixelFormat.bgrg422](https://developer.apple.com/documentation/metal/mtlpixelformat/bgrg422) |

Samplers can use any [MTLSamplerAddressMode](https://developer.apple.com/documentation/metal/mtlsampleraddressmode) to sample linear textures from this method on any device that supports the [MTLGPUFamily.apple2](https://developer.apple.com/documentation/metal/mtlgpufamily/apple2) feature family or later.

> **Note:**
>  For devices that support only the [MTLGPUFamily.apple1](https://developer.apple.com/documentation/metal/mtlgpufamily/apple1) feature family, samplers can only use [MTLSamplerAddressMode.clampToEdge](https://developer.apple.com/documentation/metal/mtlsampleraddressmode/clamptoedge) to sample a linear texture.
