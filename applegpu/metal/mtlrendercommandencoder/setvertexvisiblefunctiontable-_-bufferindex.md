# setVertexVisibleFunctionTable(_:bufferIndex:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexvisiblefunctiontable(_:bufferindex:)>

Assigns a visible function table to an entry in the vertex shader argument table.

## Declaration

```swift
func setVertexVisibleFunctionTable(_ functionTable: (any MTLVisibleFunctionTable)?, bufferIndex: Int)
```

## Parameters

- **functionTable** — An [MTLVisibleFunctionTable](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable) instance the command assigns to an entry in the vertex shader argument table for visible function tables.
- **bufferIndex** — An integer that represents the entry in the vertex shader argument table for visible function tables that stores a record of `functionTable`.

## Discussion

By default, the visible function table at each index is `nil`.

## See also

### Assigning visible function tables
- [setVertexVisibleFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexvisiblefunctiontables(_:bufferrange:)) — Assigns multiple visible function tables to a range of entries in the vertex shader argument table.
