# label

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/label>

A label for the geometry structure, suitable for debugging.

## Declaration

```swift
var label: String? { get set }
```

## See also

### Specifying base geometry properties
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/intersectionfunctiontableoffset) — An index into the intersection table for determining which intersection function Metal calls when it intersects a ray with the acceleration structure.
- [opaque](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/opaque) — A Boolean value that determines whether the geometry data in the acceleration structure needs to skip triangle-intersection tests.
- [allowDuplicateIntersectionFunctionInvocation](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/allowduplicateintersectionfunctioninvocation) — A Boolean value that indicates whether Metal calls the ray-intersection test more than once per primitive on the structure.
