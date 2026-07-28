# copy(sourceBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:destinationTexture:destinationSlice:destinationLevel:destinationOrigin:options:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcebuffer:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:destinationtexture:destinationslice:destinationlevel:destinationorigin:options:)>

Encodes a command to copy image data from a buffer into a texture with options for special texture formats.

## Declaration

```swift
func copy(sourceBuffer: any MTLBuffer, sourceOffset: Int, sourceBytesPerRow: Int, sourceBytesPerImage: Int, sourceSize: MTLSize, destinationTexture: any MTLTexture, destinationSlice: Int, destinationLevel: Int, destinationOrigin: MTLOrigin, options: MTLBlitOption = [])
```

## Parameters

- **sourceBuffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance the command copies data from.
- **sourceOffset** — A byte offset within `sourceBuffer` the command copies from. Set this value to a multiple of `destinationTexture's` pixel size, in bytes.
- **sourceBytesPerRow** — The number of bytes between adjacent rows of pixels in `sourceBuffer`. Set this value to a multiple of `destinationTexture's` pixel size, in bytes, and less than or equal to the product of `destinationTexture's` pixel size, in bytes, and the largest pixel width `destinationTexture's` type allows. If `destinationTexture` uses a compressed pixel format, set `sourceBytesPerRow` to the number of bytes between the starts of two row blocks.
- **sourceBytesPerImage** — The number of bytes between each 2D image of a 3D texture. Set this value to a multiple of `destinationTexture's` pixel size, in bytes, or `0` if `sourceSize's` [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) value is `1`.
- **sourceSize** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the size of the region in `destinationTexture`, in pixels, that the command copies data to, starting at `destinationOrigin`. Assign `1` to each dimension that’s not relevant to `destinationTexture`. If `destinationTexture` uses a compressed pixel format, set `sourceSize` to a multiple of `destinationTexture's` [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) block size. If the block extends outside the bounds of the texture, clamp `sourceSize` to the edge of the texture.
- **destinationTexture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance the command copies data to. In order to copy the contents into the destination texture, set its [isFramebufferOnly](https://developer.apple.com/documentation/metal/mtltexture/isframebufferonly) property to [false](https://developer.apple.com/documentation/Swift/false) and don’t use a combined depth/stencil [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat).
- **destinationSlice** — A slice within `destinationTexture` the command uses as its starting point for copying data to. Set this to `0` if `destinationTexture` isn’t a texture array or a cube texture.
- **destinationLevel** — A mipmap level within `destinationTexture` the command copies data to.
- **destinationOrigin** — An [MTLOrigin](https://developer.apple.com/documentation/metal/mtlorigin) instance that represents a location within `destinationTexture` that the command begins copying data to. Assign `0` to each dimension that’s not relevant to `destinationTexture`.
- **options** — An [MTLBlitOption](https://developer.apple.com/documentation/metal/mtlblitoption) value that applies to textures with applicable pixel formats, such as combined depth/stencil or PVRTC formats. If `destinationTexture's` [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) is a combined depth/stencil format, set `options` to either [depthFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/depthfromdepthstencil) or [stencilFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/stencilfromdepthstencil), but not both. If `destinationTexture's` [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) is a PVRTC format, set `options` to [rowLinearPVRTC](https://developer.apple.com/documentation/metal/mtlblitoption/rowlinearpvrtc).
