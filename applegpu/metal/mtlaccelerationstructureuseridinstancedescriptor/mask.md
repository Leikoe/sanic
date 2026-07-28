# mask

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/mask>

A mask to use for the instance when testing a ray against the geometry.

## Declaration

```swift
var mask: UInt32
```

## Discussion

Metal reserves the top 24 bits for future use.

## See also

### Customizing intersection and hit tests for the instance
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/intersectionfunctiontableoffset) — An offset for determining which function in the intersection function table Metal calls when testing a ray against the instance.
- [options](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/options) — The options for the instance.
