# offset

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/offset>

The location of an attribute in vertex data, determined by the byte offset from the start of the vertex data.

## Declaration

```swift
var offset: Int { get set }
```

## Discussion

Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions.

## See also

### Organizing the vertex attribute
- [format](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/format) — The format of the vertex attribute.
- [bufferIndex](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/bufferindex) — The index in the argument table for the associated vertex buffer.
- [MTLVertexFormat](https://developer.apple.com/documentation/metal/mtlvertexformat) — The vertex data format options for render pipelines.
