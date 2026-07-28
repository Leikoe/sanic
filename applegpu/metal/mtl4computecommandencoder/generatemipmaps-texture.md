# generateMipmaps(texture:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/generatemipmaps(texture:)>

Encodes a command that generates mipmaps for a texture instance from the base mipmap level up to the highest mipmap level.

## Declaration

```swift
func generateMipmaps(texture: any MTLTexture)
```

## Parameters

- **texture** — A mipmapped, color-renderable or color-filterable [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance the command generates mipmaps for.

## Discussion

This method generates mipmaps for a mipmapped texture. The texture you provide needs to have a [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexture/mipmaplevelcount) greater than `1`, and a color-renderable or color-filterable [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat).
