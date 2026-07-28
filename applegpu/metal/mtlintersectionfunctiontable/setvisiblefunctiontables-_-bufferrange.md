# setVisibleFunctionTables(_:bufferRange:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setvisiblefunctiontables(_:bufferrange:)>

Sets a range of visible function tables for the intersection functions.

## Declaration

```swift
func setVisibleFunctionTables(_ functionTables: [(any MTLVisibleFunctionTable)?], bufferRange: Range<Int>)
```

## Parameters

- **functionTables** — The function tables to insert.
- **bufferRange** — A range of indices in the function table’s buffer argument table.

## See also

### Specifying arguments for intersection functions
- [setBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setbuffer(_:offset:index:)) — Sets a buffer for the intersection functions.
- [setBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setbuffers(_:offsets:range:)) — Sets a range of buffers for the intersection functions.
- [setVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setvisiblefunctiontable(_:bufferindex:)) — Sets a visible function table for the intersection functions.
