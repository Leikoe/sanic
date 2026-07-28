# setTexture(_:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/settexture(_:index:)>

Encodes a reference to a texture into the argument buffer.

## Declaration

```swift
func setTexture(_ texture: (any MTLTexture)?, index: Int)
```

## Parameters

- **texture** — A texture the method encodes.
- **index** — The index of a texture within the argument buffer. The value corresponds to either the index ID of a declaration in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of an [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instance.

## See also

### Encoding textures
- [setTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/settextures(_:range:)) — Encodes references to an array of textures into the argument buffer.
