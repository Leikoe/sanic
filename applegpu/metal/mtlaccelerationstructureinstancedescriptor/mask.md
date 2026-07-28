# mask

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/mask>

A mask to use for the instance when testing a ray against the geometry.

## Declaration

```swift
var mask: UInt32
```

## Discussion

Metal reserves the top 24 bits for future use.

## See also

### Customizing intersection and hit tests for the instance
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/intersectionfunctiontableoffset) — An offset for determining which function in the intersection function table Metal needs to call when testing a ray against the instance.
- [options](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/options) — The options for the instance.
