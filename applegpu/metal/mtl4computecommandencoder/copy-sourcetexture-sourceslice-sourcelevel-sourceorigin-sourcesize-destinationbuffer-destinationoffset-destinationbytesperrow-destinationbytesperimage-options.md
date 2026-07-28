# copy(sourceTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:destinationBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:sourceorigin:sourcesize:destinationbuffer:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)>

Encodes a command that copies image data from a slice of a texture instance to a buffer, with options for special texture formats.

## Declaration

```swift
func copy(sourceTexture: any MTLTexture, sourceSlice: Int, sourceLevel: Int, sourceOrigin: MTLOrigin, sourceSize: MTLSize, destinationBuffer: any MTLBuffer, destinationOffset: Int, destinationBytesPerRow: Int, destinationBytesPerImage: Int, options: MTLBlitOption = [])
```

## Parameters

- **sourceTexture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) texture that the command copies data from. To read the source texture contents, you need to set its [isFramebufferOnly](https://developer.apple.com/documentation/metal/mtltexture/isframebufferonly) property to [false](https://developer.apple.com/documentation/Swift/false) prior to drawing into it.
- **sourceSlice** — A slice within `sourceTexture` the command uses as a starting point to copy data from. Set this to `0` if `sourceTexture` isn’t a texture array or a cube texture.
- **sourceLevel** — A mipmap level within `sourceTexture`.
- **sourceOrigin** — An [MTLOrigin](https://developer.apple.com/documentation/metal/mtlorigin) instance that represents a location within `sourceTexture` that the command begins copying data from. Assign `0` to each dimension that’s not relevant to `sourceTexture`.
- **sourceSize** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the size of the region, in pixels, that the command copies from `sourceTexture`, starting at `sourceOrigin`. Assign `1` to each dimension that’s not relevant to `sourceTexture`. If `sourceTexture` uses a compressed pixel format, set `sourceSize` to a multiple of the `sourceTexture's` [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) block size. If the block extends outside the bounds of the texture, clamp `sourceSize` to the edge of the texture.
- **destinationBuffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance the command copies data to.
- **destinationOffset** — A byte offset within `destinationBuffer` the command copies to. The value you provide as this argument needs to be a multiple of `sourceTexture's` pixel size, in bytes.
- **destinationBytesPerRow** — The number of bytes between adjacent rows of pixels in `destinationBuffer`. This value must be a multiple of `sourceTexture's` pixel size, in bytes, and less than or equal to the product of `sourceTexture's` pixel size, in bytes, and the largest pixel width `sourceTexture’s` type allows. If `sourceTexture` uses a compressed pixel format, set `destinationBytesPerRow` to the number of bytes between the starts of two row blocks.
- **destinationBytesPerImage** — The number of bytes between each 2D image of a 3D texture. This value must be a multiple of `sourceTexture's` pixel size, in bytes. Set this value to `0` if `sourceSize's` [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) value is `1`.
- **options** — A [MTLBlitOption](https://developer.apple.com/documentation/metal/mtlblitoption) value that applies to textures with applicable pixel formats, such as combined depth/stencil or PVRTC formats. If `sourceTexture's` [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) is a combined depth/stencil format, set `options` to either [depthFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/depthfromdepthstencil) or [stencilFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/stencilfromdepthstencil), but not both. If `sourceTexture's` [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) is a PVRTC format, set `options` to [rowLinearPVRTC](https://developer.apple.com/documentation/metal/mtlblitoption/rowlinearpvrtc).
