# rootResource

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/rootresource>

The resource that owns the storage for this texture.

## Declaration

```swift
var rootResource: (any MTLResource)? { get }
```

## Discussion

If the value is `nil`, then this texture image owns its own data. Otherwise, this value is the [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) instance used to create the texture. For example, it might be a texture that uses the contents of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) object or a texture view that reinterprets the contents of another [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture).

## See also

### Getting information about ancestor resources
- [parent](https://developer.apple.com/documentation/metal/mtltexture/parent) — The parent texture used to create this texture, if any.
- [parentRelativeLevel](https://developer.apple.com/documentation/metal/mtltexture/parentrelativelevel) — The base level of the parent texture used to create this texture.
- [parentRelativeSlice](https://developer.apple.com/documentation/metal/mtltexture/parentrelativeslice) — The base slice of the parent texture used to create this texture.
- [buffer](https://developer.apple.com/documentation/metal/mtltexture/buffer) — The source buffer used to create this texture, if any.
- [bufferOffset](https://developer.apple.com/documentation/metal/mtltexture/bufferoffset) — The offset in the source buffer where the texture’s data comes from.
- [bufferBytesPerRow](https://developer.apple.com/documentation/metal/mtltexture/bufferbytesperrow) — The number of bytes in each row of the texture’s source buffer.
