# setVisibleFunctionTables(_:range:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setvisiblefunctiontables(_:range:)>

Encodes references to an array of ray-tracing intersection-function tables into the argument buffer.

## Declaration

```swift
func setVisibleFunctionTables(_ visibleFunctionTables: [(any MTLVisibleFunctionTable)?], range: Range<Int>)
```

## Parameters

- **visibleFunctionTables** — An array of visible-function tables the method encodes.
- **range** — A range of indices within the argument buffer for each element in `visibleFunctionTables`. The values correspond to either the index IDs of declarations in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances.

## See also

### Encoding function tables
- [setVisibleFunctionTable(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setvisiblefunctiontable(_:index:)) — Encodes a reference to a visible-function table into the argument buffer.
- [setIntersectionFunctionTable(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setintersectionfunctiontable(_:index:)) — Encodes a reference to a ray-tracing intersection-function table into the argument buffer.
- [setIntersectionFunctionTables(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setintersectionfunctiontables(_:range:)) — Encodes references to an array of ray-tracing intersection-function tables into the argument buffer.
