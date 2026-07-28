# textureRegion

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/textureregion>

The region in the texture to update, in tiles.

## Declaration

```swift
var textureRegion: MTLRegion
```

## Discussion

When [textureLevel](https://developer.apple.com/documentation/metal/mtl4updatesparsetexturemappingoperation/texturelevel) is equal to the texture’s [firstMipmapInTail](https://developer.apple.com/documentation/metal/mtltexture/firstmipmapintail), set `origin.y` to `0` and `size.height` to `1`.
