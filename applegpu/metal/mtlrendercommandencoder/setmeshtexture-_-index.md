# setMeshTexture(_:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshtexture(_:index:)>

Assigns a texture to an entry in the mesh shader argument table.

## Declaration

```swift
func setMeshTexture(_ texture: (any MTLTexture)?, index: Int)
```

## Parameters

- **texture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance the command assigns to an entry in the mesh shader argument table for textures.
- **index** — An integer that represents the entry in the mesh shader argument table for textures that stores a record of `texture`.

## Discussion

By default, the texture at each index is `nil`.

## See also

### Assigning textures for mesh shaders
- [setMeshTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshtextures(_:range:)) — Assigns multiple textures to a range of entries in the mesh shader argument table.
