# copy(sourceTexture:destinationTexture:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:destinationtexture:)>

Encodes a command that copies data from a texture to another.

## Declaration

```swift
func copy(sourceTexture: any MTLTexture, destinationTexture: any MTLTexture)
```

## Parameters

- **sourceTexture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance the command copies data from.
- **destinationTexture** — Another [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance the command copies the data into that has the same [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) and [sampleCount](https://developer.apple.com/documentation/metal/mtltexture/samplecount) as `sourceTexture`.

## See also

### Encoding texture copy commands
- [copy(sourceTensor:sourceOrigin:sourceDimensions:destinationTensor:destinationOrigin:destinationDimensions:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetensor:sourceorigin:sourcedimensions:destinationtensor:destinationorigin:destinationdimensions:)) — Encodes a command to copy data from a slice of the data plane of a tensor into a slice of the data plane of another tensor.
- [copy(sourceTexture:sourceSlice:sourceLevel:destinationTexture:destinationSlice:destinationLevel:sliceCount:levelCount:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:destinationtexture:destinationslice:destinationlevel:slicecount:levelcount:)) — Encodes a command that copies slices of a texture to slices of another texture.
- [copy(sourceTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:destinationTexture:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:sourceorigin:sourcesize:destinationtexture:destinationslice:destinationlevel:destinationorigin:)) — Encodes a command that copies image data from a slice of a texture into a slice of another texture.
