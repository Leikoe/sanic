# setIntersectionFunctionTable(_:index:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setintersectionfunctiontable(_:index:)>

Encodes a reference to a ray-tracing intersection-function table into the argument buffer.

## Declaration

```swift
func setIntersectionFunctionTable(_ intersectionFunctionTable: (any MTLIntersectionFunctionTable)?, index: Int)
```

## Parameters

- **intersectionFunctionTable** — An intersection-function table the method encodes.
- **index** — An index of an intersection-function table within the argument buffer. The value corresponds to either the index ID of a declaration in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of an [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instance.

## See also

### Encoding function tables
- [setVisibleFunctionTable(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setvisiblefunctiontable(_:index:)) — Encodes a reference to a visible-function table into the argument buffer.
- [setIntersectionFunctionTables(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setintersectionfunctiontables(_:range:)) — Encodes references to an array of ray-tracing intersection-function tables into the argument buffer.
- [setVisibleFunctionTables(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setvisiblefunctiontables(_:range:)) — Encodes references to an array of ray-tracing intersection-function tables into the argument buffer.
