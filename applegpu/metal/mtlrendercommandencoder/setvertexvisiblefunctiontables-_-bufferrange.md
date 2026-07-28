# setVertexVisibleFunctionTables(_:bufferRange:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexvisiblefunctiontables(_:bufferrange:)>

Assigns multiple visible function tables to a range of entries in the vertex shader argument table.

## Declaration

```swift
func setVertexVisibleFunctionTables(_ functionTables: [(any MTLVisibleFunctionTable)?], bufferRange: Range<Int>)
```

## Parameters

- **functionTables** — An array of [MTLVisibleFunctionTable](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable) instances the command assigns to entries in the vertex shader argument table for visible function tables.
- **bufferRange** — A span of integers that represent the entries in the vertex shader argument table for visible function tables. Each entry stores a record of the corresponding element in `functionTables`.

## Discussion

By default, the visible function table at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setVertexVisibleFunctionTables:withBufferRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexvisiblefunctiontables:withbufferrange:).

## See also

### Assigning visible function tables
- [setVertexVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexvisiblefunctiontable(_:bufferindex:)) — Assigns a visible function table to an entry in the vertex shader argument table.
