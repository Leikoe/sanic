# setIntersectionFunctionTables(_:bufferRange:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setintersectionfunctiontables(_:bufferrange:)>

Binds multiple intersection function tables to the buffer argument table, allowing you to call their functions on the GPU.

## Declaration

```swift
func setIntersectionFunctionTables(_ intersectionFunctionTables: [(any MTLIntersectionFunctionTable)?], bufferRange: Range<Int>)
```

## Parameters

- **intersectionFunctionTables** — An array of [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) instances to bind.
- **bufferRange** — The argument buffer table indices to bind each of the `intersectionFunctionTables` to, in the order they appear.

## Discussion

> **Warning:**
>  This method requires that the number of instances in `visibleFunctionTables` be the same as the length of `bufferRange`.

## See also

### Binding function tables
- [setVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setvisiblefunctiontable(_:bufferindex:)) — Binds a visible function table to the buffer argument table, allowing you to call its functions on the GPU.
- [setVisibleFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setvisiblefunctiontables(_:bufferrange:)) — Binds multiple visible function tables to the buffer argument table, allowing you to call their functions on the GPU.
