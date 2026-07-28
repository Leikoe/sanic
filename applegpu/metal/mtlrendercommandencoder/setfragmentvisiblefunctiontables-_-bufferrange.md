# setFragmentVisibleFunctionTables(_:bufferRange:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentvisiblefunctiontables(_:bufferrange:)>

Assigns multiple visible function tables to a range of entries in the fragment shader argument table.

## Declaration

```swift
func setFragmentVisibleFunctionTables(_ functionTables: [(any MTLVisibleFunctionTable)?], bufferRange: Range<Int>)
```

## Parameters

- **functionTables** — An array of [MTLVisibleFunctionTable](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable) instances the command assigns to entries in the fragment shader argument table for visible function tables.
- **bufferRange** — A span of integers that represent the entries in the fragment shader argument table for visible function tables. Each entry stores a record of the corresponding element in `functionTables`.

## Discussion

By default, the visible function table at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setFragmentVisibleFunctionTables:withBufferRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentvisiblefunctiontables:withbufferrange:).

## See also

### Assigning visible function tables
- [setFragmentVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentvisiblefunctiontable(_:bufferindex:)) — Assigns a visible function table to an entry in the fragment shader argument table.
