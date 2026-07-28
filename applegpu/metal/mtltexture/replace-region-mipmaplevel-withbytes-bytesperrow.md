# replace(region:mipmapLevel:withBytes:bytesPerRow:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:withbytes:bytesperrow:)>

Copies a block of pixels into a section of texture slice 0.

## Declaration

```swift
func replace(region: MTLRegion, mipmapLevel level: Int, withBytes pixelBytes: UnsafeRawPointer, bytesPerRow: Int)
```

## Parameters

- **region** — The location of a block of pixels in the texture slice. The region needs to be within the dimensions of the slice.
- **level** — A zero-indexed value that specifies which mipmap level is the destination. If the texture doesn’t have mipmaps, use `0`.
- **pixelBytes** — A pointer to the bytes in memory to copy.
- **bytesPerRow** — The stride, in bytes, of one row in the source data. For [MTLTextureType.type1D](https://developer.apple.com/documentation/metal/mtltexturetype/type1d) and [MTLTextureType.type1DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type1darray), use `0`. For raw and packed pixel types, the stride is the number of pixels in one row. For compressed pixel formats, the stride is the number of bytes from the beginning of one row of blocks to the beginning of the next. When source data consists of only a single row, use `0`. Your data type determines how you should compute `bytesPerRow`: - For raw or packed pixel data, use a value greater than or equal to the size of data in one row, and less than [max](https://developer.apple.com/documentation/Swift/Int32/max) `* pixel size`. - For compressed pixel data, use a multiple of the compression block size. When working with PowerVR Texture Compression (PVRTC), use `0.` Nonzero values smaller than the texture width or not a multiple of the pixel size cause an error.

## Discussion

This method runs on the CPU and immediately copies the pixel data into the texture. It doesn’t synchronize against any GPU memory operations to the texture. Ensure all operations that write or render to the texture complete before reading the texture’s contents using one of the following methods:

- Synchronize on the GPU with a [synchronize(resource:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(resource:)) or [synchronize(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(texture:slice:level:)) command in an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder).

- Synchronize on the CPU with a callback passed to the [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) method to handle completion asynchronously, or the [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilcompleted()) method to block thread execution until the GPU work completes.

If the texture image has a compressed pixel format, only write to block-aligned regions. If the size of a dimension of region isn’t a multiple of the block size, then include both the edge block and space up to the texture dimensions in `bytesPerRow`.

To copy your data to a private texture, copy your data to a temporary texture with non-private storage, and then use an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) to copy the data to the private texture for GPU use.

## See also

### Related Documentation
- [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) — The data formats that describe the organization and characteristics of individual pixels in a texture.

### Copying data into a texture image
- [replace(region:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:slice:withbytes:bytesperrow:bytesperimage:)) — Copies pixel data into a section of a texture slice.
