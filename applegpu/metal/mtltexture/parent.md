# parent

*Instance Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/parent>

The parent texture used to create this texture, if any.

## Declaration

```swift
var parent: (any MTLTexture)? { get }
```

## Discussion

When this value is `nil`, an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance provides texture data.

## See also

### Getting information about ancestor resources
- [parentRelativeLevel](https://developer.apple.com/documentation/metal/mtltexture/parentrelativelevel) — The base level of the parent texture used to create this texture.
- [parentRelativeSlice](https://developer.apple.com/documentation/metal/mtltexture/parentrelativeslice) — The base slice of the parent texture used to create this texture.
- [buffer](https://developer.apple.com/documentation/metal/mtltexture/buffer) — The source buffer used to create this texture, if any.
- [bufferOffset](https://developer.apple.com/documentation/metal/mtltexture/bufferoffset) — The offset in the source buffer where the texture’s data comes from.
- [bufferBytesPerRow](https://developer.apple.com/documentation/metal/mtltexture/bufferbytesperrow) — The number of bytes in each row of the texture’s source buffer.
- [rootResource](https://developer.apple.com/documentation/metal/mtltexture/rootresource) — The resource that owns the storage for this texture.
