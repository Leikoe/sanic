# intersectionFunctionTableOffset

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/intersectionfunctiontableoffset>

An index into the intersection table for determining which intersection function Metal calls when it intersects a ray with the acceleration structure.

## Declaration

```swift
var intersectionFunctionTableOffset: Int { get set }
```

## See also

### Specifying base geometry properties
- [label](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/label) — A label for the geometry structure, suitable for debugging.
- [opaque](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/opaque) — A Boolean value that determines whether the geometry data in the acceleration structure needs to skip triangle-intersection tests.
- [allowDuplicateIntersectionFunctionInvocation](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/allowduplicateintersectionfunctioninvocation) — A Boolean value that indicates whether Metal calls the ray-intersection test more than once per primitive on the structure.
