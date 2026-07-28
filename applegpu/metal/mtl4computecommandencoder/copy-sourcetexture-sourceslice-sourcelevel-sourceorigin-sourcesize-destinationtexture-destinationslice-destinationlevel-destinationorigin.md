# copy(sourceTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:destinationTexture:destinationSlice:destinationLevel:destinationOrigin:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:sourceorigin:sourcesize:destinationtexture:destinationslice:destinationlevel:destinationorigin:)>

Encodes a command that copies image data from a slice of a texture into a slice of another texture.

## Declaration

```swift
func copy(sourceTexture: any MTLTexture, sourceSlice: Int, sourceLevel: Int, sourceOrigin: MTLOrigin, sourceSize: MTLSize, destinationTexture: any MTLTexture, destinationSlice: Int, destinationLevel: Int, destinationOrigin: MTLOrigin)
```

## Parameters

- **sourceTexture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) texture that the command copies data from. To read the source texture contents, you need to set its [isFramebufferOnly](https://developer.apple.com/documentation/metal/mtltexture/isframebufferonly) property to [false](https://developer.apple.com/documentation/Swift/false) prior to drawing into it.
- **sourceSlice** — A slice within `sourceTexture` the command uses as a starting point to copy data from. Set this to `0` if `sourceTexture` isn’t a texture array or a cube texture.
- **sourceLevel** — A mipmap level within `sourceTexture`.
- **sourceOrigin** — An [MTLOrigin](https://developer.apple.com/documentation/metal/mtlorigin) instance that represents a location within `sourceTexture` that the command begins copying data from. Assign `0` to each dimension that’s not relevant to `sourceTexture`.
- **sourceSize** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the size of the region, in pixels, that the command copies from `sourceTexture`, starting at `sourceOrigin`. Assign `1` to each dimension that’s not relevant to `sourceTexture`. If sourceTexture uses a compressed pixel format, set `sourceSize` to a multiple of the pixel format’s block size. If the block extends outside the bounds of the texture, clamp `sourceSize` to the edge of the texture.
- **destinationTexture** — Another [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) the command copies the data to that has the same [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) and [sampleCount](https://developer.apple.com/documentation/metal/mtltexture/samplecount) as `sourceTexture`. To write the contents into this texture, you need to set its [isFramebufferOnly](https://developer.apple.com/documentation/metal/mtltexture/isframebufferonly) property to [false](https://developer.apple.com/documentation/Swift/false).
- **destinationSlice** — A slice within `destinationTexture` the command uses as its starting point for copying data to. Set this to `0` if `destinationTexture` isn’t a texture array or a cube texture.
- **destinationLevel** — A mipmap level within `destinationTexture`. The mipmap level you reference needs to have the same size as the `sourceTexture` slice’s mipmap at `sourceLevel`.
- **destinationOrigin** — An [MTLOrigin](https://developer.apple.com/documentation/metal/mtlorigin) instance that represents a location within `destinationTexture` that the command begins copying data to. Assign `0` to each dimension that’s not relevant to `destinationTexture`.

## See also

### Encoding texture copy commands
- [copy(sourceTensor:sourceOrigin:sourceDimensions:destinationTensor:destinationOrigin:destinationDimensions:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetensor:sourceorigin:sourcedimensions:destinationtensor:destinationorigin:destinationdimensions:)) — Encodes a command to copy data from a slice of the data plane of a tensor into a slice of the data plane of another tensor.
- [copy(sourceTexture:destinationTexture:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:destinationtexture:)) — Encodes a command that copies data from a texture to another.
- [copy(sourceTexture:sourceSlice:sourceLevel:destinationTexture:destinationSlice:destinationLevel:sliceCount:levelCount:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:destinationtexture:destinationslice:destinationlevel:slicecount:levelcount:)) — Encodes a command that copies slices of a texture to slices of another texture.
