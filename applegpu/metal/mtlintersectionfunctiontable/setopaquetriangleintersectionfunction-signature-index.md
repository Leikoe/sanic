# setOpaqueTriangleIntersectionFunction(signature:index:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setopaquetriangleintersectionfunction(signature:index:)>

Sets an entry in the intersection table to point to a system-defined opaque triangle intersection function.

## Declaration

```swift
func setOpaqueTriangleIntersectionFunction(signature: MTLIntersectionFunctionSignature, index: Int)
```

## Parameters

- **signature** — The signature of the function.
- **index** — The index in the table to change.

## See also

### Specifying opaque triangle intersection testing
- [setOpaqueTriangleIntersectionFunction(signature:range:)](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable/setopaquetriangleintersectionfunction(signature:range:)) — Sets a range of entries in the intersection table to point to a system-defined opaque triangle intersection function.
