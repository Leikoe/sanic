# setTileTextures(_:range:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settiletextures(_:range:)>

Assigns multiple textures to a range of entries in the tile shader argument table.

## Declaration

```swift
func setTileTextures(_ textures: [(any MTLTexture)?], range: Range<Int>)
```

## Parameters

- **textures** — An array of [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instances the command assigns to entries in the tile shader argument table for textures.
- **range** — A span of integers that represent the entries in the tile shader argument table for textures. Each entry stores a record of the corresponding element in `textures`.

## Discussion

By default, the texture at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setTileTextures:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settiletextures:withrange:).

## See also

### Assigning textures
- [setTileTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settiletexture(_:index:)) — Assigns a texture to an entry in the tile shader argument table.
