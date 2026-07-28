# access

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexturereferencetype/access>

The texture’s read/write access to the argument.

## Declaration

```swift
var access: MTLBindingAccess { get }
```

## Discussion

This property indicates the type of access qualifiers (read-only, write-only, or read-write) used in the Metal shading language code. For information on possible values, see [MTLArgumentAccess](https://developer.apple.com/documentation/metal/mtlargumentaccess).

## See also

### Describing the texture
- [textureType](https://developer.apple.com/documentation/metal/mtltexturereferencetype/texturetype) — The texture type of the texture.
- [textureDataType](https://developer.apple.com/documentation/metal/mtltexturereferencetype/texturedatatype) — The data type of the texture.
- [isDepthTexture](https://developer.apple.com/documentation/metal/mtltexturereferencetype/isdepthtexture) — A Boolean value that indicates whether the texture is a depth texture.
