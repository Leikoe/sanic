# destinationOrigin

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/destinationorigin>

The origin in the destination texture to copy into, in tiles.

## Declaration

```swift
var destinationOrigin: MTLOrigin
```

## Discussion

The X, Y and Z coordinates of the tiles relative to the origin match the same coordinates in the source region.

When [destinationLevel](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/destinationlevel) is equal to the destination texture’s [firstMipmapInTail](https://developer.apple.com/documentation/metal/mtltexture/firstmipmapintail), set `destinationOrigin.y` to `0`.
