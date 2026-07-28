# setTextures(_:range:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 8.0, macOS 10.11, tvOS 8.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/settextures(_:range:)>

Binds multiple textures to the texture argument table, allowing compute functions to access their data on the GPU.

## Declaration

```swift
func setTextures(_ textures: [(any MTLTexture)?], range: Range<Int>)
```

## Parameters

- **textures** — A list of [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instances to bind to the texture argument table.
- **range** — The texture table indices to bind each of the `textures` to, in the order they appear.

## Discussion

> **Important:**
>  This method requires that the number of instances in `textures` be the same as the length of `range`.

## See also

### Binding textures
- [setTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/settexture(_:index:)) — Binds a texture to the texture argument table, allowing compute kernels to access its data on the GPU.
