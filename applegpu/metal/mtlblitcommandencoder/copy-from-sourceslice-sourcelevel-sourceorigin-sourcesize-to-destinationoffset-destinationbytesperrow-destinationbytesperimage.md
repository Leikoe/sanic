# copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:)>

Encodes a command that copies image data from a texture slice to a buffer.

## Declaration

```swift
func copy(from sourceTexture: any MTLTexture, sourceSlice: Int, sourceLevel: Int, sourceOrigin: MTLOrigin, sourceSize: MTLSize, to destinationBuffer: any MTLBuffer, destinationOffset: Int, destinationBytesPerRow: Int, destinationBytesPerImage: Int)
```

## Parameters

- **sourceTexture** — A texture with an [isFramebufferOnly](https://developer.apple.com/documentation/metal/mtltexture/isframebufferonly) property value of [false](https://developer.apple.com/documentation/Swift/false) that the command copies data from.
- **sourceSlice** — A slice within `sourceTexture`. For textures that use a combined depth/stencil pixel format, call the [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)) method instead. Configure that method’s `options` parameter appropriately.
- **sourceLevel** — A mipmap level within `sourceTexture`.
- **sourceOrigin** — A location within `sourceTexture` that the command begins copying data from. Assign `0` to each dimension that’s not relevant to `sourceTexture`. For example: - If the source texture is a 2D texture, set the origin’s [z](https://developer.apple.com/documentation/metal/mtlorigin/z) property to `0`. - If the source texture is a 1D texture, set the origin’s [y](https://developer.apple.com/documentation/metal/mtlorigin/y) and [z](https://developer.apple.com/documentation/metal/mtlorigin/z) properties to `0`.
- **sourceSize** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance, which can represent a 3D region, that instructs the command how many pixels to copy from `sourceTexture`, starting at `sourceOrigin`. Assign `1` to each dimension that’s not relevant to `sourceTexture`. For example: - If the source texture is a 2D texture, set the size’s [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) property to `1`. - If the source texture is a 1D texture, set the size’s [height](https://developer.apple.com/documentation/metal/mtlsize/height) and [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) properties to `1`. If `sourceTexture` uses a compressed pixel format, set `sourceSize` to a multiple of the pixel format’s block size. If the block extends outside the bounds of the texture, clamp `sourceSize` to the edge of the texture.
- **destinationBuffer** — A buffer the command copies data to.
- **destinationOffset** — A byte offset within `destinationBuffer` the command copies to, which needs to be a multiple of the source texture’s pixel size, in bytes.
- **destinationBytesPerRow** — The number of bytes between adjacent rows of pixels in the destination buffer’s memory, which needs to be: - A multiple of the source texture’s pixel size, in bytes - Less than or equal to the product of the source texture’s pixel size, in bytes, and the largest pixel width the source texture’s type allows If `sourceTexture` uses a compressed pixel format, set `destinationBytesPerRow` to the number of bytes between the starts of two row blocks.
- **destinationBytesPerImage** — The number of bytes between each 2D image of a 3D texture. This value needs to be a multiple of the source texture’s pixel size, in bytes. Set this value to `0` for 2D textures, which means `sourceSize.`[depth](https://developer.apple.com/documentation/metal/mtlsize/depth) is equal to `1`.

## Discussion

This method is the equivalent of passing an empty [OptionSet](https://developer.apple.com/documentation/Swift/OptionSet) to the `options` parameter of [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)). In Swift, pass `[]` to represent an empty option set, and in Objective-C, pass [MTLBlitOptionNone](https://developer.apple.com/documentation/metal/mtlblitoption/mtlblitoptionnone).

> **Important:**
>  If the pixel format of `sourceTexture` is a PVRTC format, use [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)) instead.

## See also

### Copying texture data to a buffer
- [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)) — Encodes a command that copies image data from a texture slice to a buffer, and provides options for special texture formats.
