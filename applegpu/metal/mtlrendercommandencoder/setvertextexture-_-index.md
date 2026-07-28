# setVertexTexture(_:index:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertextexture(_:index:)>

Assigns a texture to an entry in the vertex shader argument table.

## Declaration

```swift
func setVertexTexture(_ texture: (any MTLTexture)?, index: Int)
```

## Parameters

- **texture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance the command assigns to an entry in the vertex shader argument table for textures.
- **index** — An integer that represents the entry in the vertex shader argument table for textures that stores a record of `texture`.

## Discussion

By default, the texture at each index is `nil`.

## See also

### Assigning textures
- [setVertexTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertextextures(_:range:)) — Assigns multiple textures to a range of entries in the vertex shader argument table.
