# bufferOffset

*Instance Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.12, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/bufferoffset>

The offset in the source buffer where the texture’s data comes from.

## Declaration

```swift
var bufferOffset: Int { get }
```

## Discussion

This property is only valid for textures created from a [buffer](https://developer.apple.com/documentation/metal/mtltexture/buffer). The default value is `0`.

## See also

### Related Documentation
- [makeTexture(descriptor:offset:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtlbuffer/maketexture(descriptor:offset:bytesperrow:)) — Creates a texture that shares its storage with the buffer.

### Getting information about ancestor resources
- [parent](https://developer.apple.com/documentation/metal/mtltexture/parent) — The parent texture used to create this texture, if any.
- [parentRelativeLevel](https://developer.apple.com/documentation/metal/mtltexture/parentrelativelevel) — The base level of the parent texture used to create this texture.
- [parentRelativeSlice](https://developer.apple.com/documentation/metal/mtltexture/parentrelativeslice) — The base slice of the parent texture used to create this texture.
- [buffer](https://developer.apple.com/documentation/metal/mtltexture/buffer) — The source buffer used to create this texture, if any.
- [bufferBytesPerRow](https://developer.apple.com/documentation/metal/mtltexture/bufferbytesperrow) — The number of bytes in each row of the texture’s source buffer.
- [rootResource](https://developer.apple.com/documentation/metal/mtltexture/rootresource) — The resource that owns the storage for this texture.
