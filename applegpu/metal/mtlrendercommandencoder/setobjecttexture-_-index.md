# setObjectTexture(_:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjecttexture(_:index:)>

Assigns a texture to an entry in the object shader argument table.

## Declaration

```swift
func setObjectTexture(_ texture: (any MTLTexture)?, index: Int)
```

## Parameters

- **texture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance the command assigns to an entry in the object shader argument table for textures.
- **index** — An integer that represents the entry in the object shader argument table for textures that stores a record of `texture`.

## Discussion

By default, the texture at each index is `nil`.

## See also

### Assigning textures for object shaders
- [setObjectTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjecttextures(_:range:)) — Assigns multiple textures to a range of entries in the object shader argument table.
