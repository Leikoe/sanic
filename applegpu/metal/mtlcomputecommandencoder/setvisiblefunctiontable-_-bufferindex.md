# setVisibleFunctionTable(_:bufferIndex:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setvisiblefunctiontable(_:bufferindex:)>

Binds a visible function table to the buffer argument table, allowing you to call its functions on the GPU.

## Declaration

```swift
func setVisibleFunctionTable(_ visibleFunctionTable: (any MTLVisibleFunctionTable)?, bufferIndex: Int)
```

## Parameters

- **visibleFunctionTable** — The [MTLVisibleFunctionTable](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable) to bind.
- **bufferIndex** — The index the function table binds to in the buffer argument table.

## See also

### Binding function tables
- [setVisibleFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setvisiblefunctiontables(_:bufferrange:)) — Binds multiple visible function tables to the buffer argument table, allowing you to call their functions on the GPU.
- [setIntersectionFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setintersectionfunctiontables(_:bufferrange:)) — Binds multiple intersection function tables to the buffer argument table, allowing you to call their functions on the GPU.
