# sourceRegion

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/sourceregion>

The region in the source texture, in tiles.

## Declaration

```swift
var sourceRegion: MTLRegion
```

## Discussion

The tiles remain mapped in the source texture.

When [sourceLevel](https://developer.apple.com/documentation/metal/mtl4copysparsetexturemappingoperation/sourcelevel) is equal to the source texture’s [firstMipmapInTail](https://developer.apple.com/documentation/metal/mtltexture/firstmipmapintail), set `origin.y` to `0` and `size.height` to `1`.
