# setMeshTextures(_:range:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshtextures(_:range:)>

Assigns multiple textures to a range of entries in the mesh shader argument table.

## Declaration

```swift
func setMeshTextures(_ textures: [(any MTLTexture)?], range: Range<Int>)
```

## Parameters

- **textures** — An array of [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instances the command assigns to entries in the mesh shader argument table for textures.
- **range** — A span of integers that represent the entries in the mesh shader argument table for textures. Each entry stores a record of the corresponding element in `textures`.

## Discussion

By default, the texture at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setMeshTextures:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshtextures:withrange:).

## See also

### Assigning textures for mesh shaders
- [setMeshTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshtexture(_:index:)) — Assigns a texture to an entry in the mesh shader argument table.
