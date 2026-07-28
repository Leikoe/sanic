# copy(from:to:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:to:)>

Encodes a command that copies data from one texture to another.

## Declaration

```swift
func copy(from sourceTexture: any MTLTexture, to destinationTexture: any MTLTexture)
```

## Parameters

- **sourceTexture** — A texture the command copies data from.
- **destinationTexture** — Another texture the command copies the data to that has the same pixel format and sample count as `sourceTexture`.

## Discussion

The textures can be different sizes as long as the larger texture has a mipmap level that’s the same size as the smaller texture’s level `0` mipmap.

The command copies all identical mipmap sizes. If both textures are arrays, the command copies as many texture slices (array elements) as possible.

## See also

### Copying texture data to another texture
- [copy(from:sourceSlice:sourceLevel:to:destinationSlice:destinationLevel:sliceCount:levelCount:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:to:destinationslice:destinationlevel:slicecount:levelcount:)) — Encodes a command that copies slices of a texture to another texture’s slices.
- [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationslice:destinationlevel:destinationorigin:)) — Encodes a command that copies image data from a texture’s slice into another slice.
- [copy(from:sourceOrigin:sourceDimensions:to:destinationOrigin:destinationDimensions:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceorigin:sourcedimensions:to:destinationorigin:destinationdimensions:)) — Encodes a command to copy data from a slice of the data plane of a tensor into a slice of the data plane of another tensor.
