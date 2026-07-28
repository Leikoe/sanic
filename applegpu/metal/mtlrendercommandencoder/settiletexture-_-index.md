# setTileTexture(_:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settiletexture(_:index:)>

Assigns a texture to an entry in the tile shader argument table.

## Declaration

```swift
func setTileTexture(_ texture: (any MTLTexture)?, index: Int)
```

## Parameters

- **texture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance the command assigns to an entry in the tile shader argument table for textures.
- **index** — An integer that represents the entry in the tile shader argument table for textures that stores a record of `texture`.

## Discussion

By default, the texture at each index is `nil`.

## See also

### Assigning textures
- [setTileTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settiletextures(_:range:)) — Assigns multiple textures to a range of entries in the tile shader argument table.
