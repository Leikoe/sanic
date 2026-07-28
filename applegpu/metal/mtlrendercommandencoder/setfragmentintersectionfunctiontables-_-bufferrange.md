# setFragmentIntersectionFunctionTables(_:bufferRange:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentintersectionfunctiontables(_:bufferrange:)>

Assigns multiple intersection function tables to a range of entries in the fragment shader argument table.

## Declaration

```swift
func setFragmentIntersectionFunctionTables(_ functionTables: [(any MTLIntersectionFunctionTable)?], bufferRange: Range<Int>)
```

## Parameters

- **functionTables** — An array of [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) instances the command assigns to entries in the fragment shader argument table for intersection function tables.
- **bufferRange** — A span of integers that represent the entries in the fragment shader argument table for intersection function tables. Each entry stores a record of the corresponding element in `functionTables`.

## Discussion

By default, the intersection function table at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setFragmentIntersectionFunctionTables:withBufferRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentintersectionfunctiontables:withbufferrange:).

## See also

### Assigning intersection function tables
- [setFragmentIntersectionFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentintersectionfunctiontable(_:bufferindex:)) — Assigns an intersection function table to an entry in the fragment shader argument table.
