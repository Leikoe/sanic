# textureType

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargument/texturetype>

The texture type of a texture argument.

## Declaration

```swift
var textureType: MTLTextureType { get }
```

## Discussion

For information on possible values, see [MTLTextureType](https://developer.apple.com/documentation/metal/mtltexturetype). If the argument is not a texture, querying this property is a fatal error.

## See also

### Describing a texture argument
- [textureDataType](https://developer.apple.com/documentation/metal/mtlargument/texturedatatype) — The data type of a texture argument.
- [isDepthTexture](https://developer.apple.com/documentation/metal/mtlargument/isdepthtexture) — A Boolean value that indicates whether the texture is a depth texture.
