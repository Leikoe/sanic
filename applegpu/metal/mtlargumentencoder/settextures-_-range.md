# setTextures(_:range:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 11.0, macOS 10.13, tvOS 11.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/settextures(_:range:)>

Encodes references to an array of textures into the argument buffer.

## Declaration

```swift
func setTextures(_ textures: [(any MTLTexture)?], range: Range<Int>)
```

## Parameters

- **textures** — An array of textures the method encodes.
- **range** — A range of indices within the argument buffer for each element in `textures`. The values correspond to either the index IDs of declarations in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances.

## See also

### Encoding textures
- [setTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/settexture(_:index:)) — Encodes a reference to a texture into the argument buffer.
