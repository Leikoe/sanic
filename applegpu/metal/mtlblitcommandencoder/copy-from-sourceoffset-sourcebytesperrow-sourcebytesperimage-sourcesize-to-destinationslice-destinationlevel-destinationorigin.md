# copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:)>

Encodes a command to copy image data from a source buffer into a destination texture.

## Declaration

```swift
func copy(from sourceBuffer: any MTLBuffer, sourceOffset: Int, sourceBytesPerRow: Int, sourceBytesPerImage: Int, sourceSize: MTLSize, to destinationTexture: any MTLTexture, destinationSlice: Int, destinationLevel: Int, destinationOrigin: MTLOrigin)
```

## Parameters

- **sourceBuffer** — A buffer the command copies data from.
- **sourceOffset** — A byte offset within `sourceBuffer` that the command copies from, which needs to be a multiple of the destination texture’s pixel size, in bytes.
- **sourceBytesPerRow** — The number of bytes between adjacent rows of pixels in the source buffer’s memory, which needs to be: - A multiple of the source texture’s pixel size, in bytes - Less than or equal to the product of the destination texture’s pixel size, in bytes, and the largest pixel width the destination texture’s type allows If `destinationTexture` uses a compressed pixel format, set `sourceBytesPerRow` to the number of bytes between the starts of two row blocks.
- **sourceBytesPerImage** — The number of bytes between each 2D image of a 3D texture. This value needs to be a multiple of the source texture’s pixel size, in bytes. Set this value to `0` for 2D textures, which means `sourceSize.`[depth](https://developer.apple.com/documentation/metal/mtlsize/depth) is equal to `1`.
- **sourceSize** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance, which can represent a 3D region, that instructs the command how many pixels to copy to `destinationTexture`, starting at `destinationOrigin`. Assign `1` to each dimension that’s not relevant to `destinationTexture`. For example: - If the destination texture is a 2D texture, set the size’s [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) property to `1`. - If the destination texture is a 1D texture, set the size’s [height](https://developer.apple.com/documentation/metal/mtlsize/height) and [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) properties to `1`. If `destinationTexture` uses a compressed pixel format, set `sourceSize` to a multiple of the pixel format’s block size. If the block extends outside the bounds of the texture, clamp `sourceSize` to the edge of the texture.
- **destinationTexture** — A texture with an [isFramebufferOnly](https://developer.apple.com/documentation/metal/mtltexture/isframebufferonly) property value of [false](https://developer.apple.com/documentation/Swift/false) that the command copies data to.
- **destinationSlice** — A slice within `destinationTexture`. For textures that use a combined depth/stencil pixel format, call the [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:options:)) method instead. Configure that method’s `options` parameter appropriately.
- **destinationLevel** — A mipmap level within `destinationTexture`.
- **destinationOrigin** — A location within `destinationTexture` that the command begins copying data to. Assign `0` to each dimension that’s not relevant to `destinationTexture`. For example: - If the destination texture is a 2D texture, set the origin’s [z](https://developer.apple.com/documentation/metal/mtlorigin/z) property to `0`. - If the destination texture is a 1D texture, set the origin’s [y](https://developer.apple.com/documentation/metal/mtlorigin/y) and [z](https://developer.apple.com/documentation/metal/mtlorigin/z) properties to `0`.

## Discussion

This method is the equivalent of passing an empty [OptionSet](https://developer.apple.com/documentation/Swift/OptionSet) to the `options` parameter of [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:options:)). In Swift, pass `[]` to represent an empty option set, and in Objective-C, pass [MTLBlitOptionNone](https://developer.apple.com/documentation/metal/mtlblitoption/mtlblitoptionnone).

> **Important:**
>  If the pixel format of `sourceTexture` is a PVRTC format, use [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:options:)) instead.

## See also

### Copying buffer data to a texture
- [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:options:)) — Encodes a command to copy image data from a source buffer into a destination texture.
