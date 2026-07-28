# bufferIndex

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/bufferindex>

The index in the argument table for the associated vertex buffer.

## Declaration

```swift
var bufferIndex: Int { get set }
```

## See also

### Related Documentation
- [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:)) — Assigns a buffer to an entry in the vertex shader argument table.

### Organizing the vertex attribute
- [format](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/format) — The format of the vertex attribute.
- [offset](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/offset) — The location of an attribute in vertex data, determined by the byte offset from the start of the vertex data.
- [MTLVertexFormat](https://developer.apple.com/documentation/metal/mtlvertexformat) — The vertex data format options for render pipelines.
