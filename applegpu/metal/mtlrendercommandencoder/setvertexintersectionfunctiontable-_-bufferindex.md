# setVertexIntersectionFunctionTable(_:bufferIndex:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexintersectionfunctiontable(_:bufferindex:)>

Assigns an intersection function table to an entry in the vertex shader argument table.

## Declaration

```swift
func setVertexIntersectionFunctionTable(_ intersectionFunctionTable: (any MTLIntersectionFunctionTable)?, bufferIndex: Int)
```

## Parameters

- **intersectionFunctionTable** — An [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) instance the command assigns to an entry in the vertex shader argument table for intersection function tables.
- **bufferIndex** — An integer that represents the entry in the vertex shader argument table for intersection function tables that stores a record of `intersectionFunctionTable`.

## Discussion

By default, the intersection function table at each index is `nil`.

## See also

### Assigning intersection function tables
- [setVertexIntersectionFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexintersectionfunctiontables(_:bufferrange:)) — Assigns multiple intersection function tables to a range of entries in the vertex shader argument table.
