# setTexture(_:index:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/settexture(_:index:)>

Binds a texture to the texture argument table, allowing compute kernels to access its data on the GPU.

## Declaration

```swift
func setTexture(_ texture: (any MTLTexture)?, index: Int)
```

## Parameters

- **texture** — An [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance to bind to the texture argument table.
- **index** — The index the texture binds to in the texture argument table.

## See also

### Binding textures
- [setTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/settextures(_:range:)) — Binds multiple textures to the texture argument table, allowing compute functions to access their data on the GPU.
