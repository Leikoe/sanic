# generateMipmaps(for:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/generatemipmaps(for:)>

Encodes a command that generates mipmaps for a texture from the base mipmap level up to the highest mipmap level.

## Declaration

```swift
func generateMipmaps(for texture: any MTLTexture)
```

## Parameters

- **texture** — A texture instance the command generates mipmaps for that has: - A [mipmapLevelCount](https://developer.apple.com/documentation/metal/mtltexture/mipmaplevelcount) property that’s greater than `1` - A [pixelFormat](https://developer.apple.com/documentation/metal/mtltexture/pixelformat) that’s color-renderable and color-filterable

## Discussion

The command generates with scaled images for all levels up to the highest mipmap level.

> **Note:**
>  The image filtering that GPU drivers use to generate the mipmaps may vary by the feature families ([MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily)) it supports.
