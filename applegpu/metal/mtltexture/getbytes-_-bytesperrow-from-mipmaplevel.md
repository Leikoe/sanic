# getBytes(_:bytesPerRow:from:mipmapLevel:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/getbytes(_:bytesperrow:from:mipmaplevel:)>

Copies pixel data from the first slice of the texture to a buffer in system memory.

## Declaration

```swift
func getBytes(_ pixelBytes: UnsafeMutableRawPointer, bytesPerRow: Int, from region: MTLRegion, mipmapLevel level: Int)
```

## Parameters

- **pixelBytes** — A pointer to a destination buffer in system memory.
- **bytesPerRow** — The number of bytes (*stride*) between two adjacent rows of pixel data in the destination buffer. For [MTLTextureType.type1D](https://developer.apple.com/documentation/metal/mtltexturetype/type1d) and [MTLTextureType.type1DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type1darray), use `0`. For raw and packed pixel types, the stride is the number of pixels in one row. For compressed pixel formats, the stride is the number of bytes from the beginning of one row of blocks to the beginning of the next. Your data type determines how you should compute `bytesPerRow`: - For raw or packed pixel data, use a multiple of the pixel size less than [max](https://developer.apple.com/documentation/Swift/Int32/max) `* pixel size`. - For compressed pixel data, use a multiple of the compression block size. When working with PowerVR Texture Compression (PVRTC), use `0.` Nonzero values smaller than the texture width or any values not a multiple of the pixel or block size cause an error.
- **region** — The location of a block of pixels in the texture slice. For textures compressed as PVRTC, use the entire texture for the region.
- **level** — A zero-indexed value that selects the texture’s mipmap level as the method’s data source. Use `0` for textures that don’t have mipmaps.

## Discussion

> **Important:**
>  Don’t use this method for textures where [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) is [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private). Instead, copy data from the private texture with an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) to another texture accessible from the CPU, and then call this method on the accessible texture.

This method runs on the CPU and immediately copies the pixel data from the texture to system memory, but it doesn’t synchronize with any GPU texture memory operations. Ensure all operations that write or render to the texture complete before reading the texture’s contents using one of the following methods:

- Synchronize on the GPU with a [synchronize(resource:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(resource:)) or [synchronize(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(texture:slice:level:)) command in an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder).

- Synchronize on the CPU with a callback passed to the [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) method to handle completion asynchronously, or the [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilcompleted()) method to block thread execution until the GPU work completes.

For multisample textures, the method consecutively positions each sample within a pixel in memory and treats the pixels as part of one row.

## See also

### Copying data from a texture image
- [getBytes(_:bytesPerRow:bytesPerImage:from:mipmapLevel:slice:)](https://developer.apple.com/documentation/metal/mtltexture/getbytes(_:bytesperrow:bytesperimage:from:mipmaplevel:slice:)) — Copies pixel data from the texture to a buffer in system memory.
