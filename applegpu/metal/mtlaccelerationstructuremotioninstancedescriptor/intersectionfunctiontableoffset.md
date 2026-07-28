# intersectionFunctionTableOffset

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/intersectionfunctiontableoffset>

An offset into the intersection-function table for ray tracing, which applies to the next acceleration-structure motion instance you create with the descriptor.

## Declaration

```swift
var intersectionFunctionTableOffset: UInt32
```

## Discussion

By default, after Metal finds an intersection between a ray and a primitive, it runs your specified intersection function to determine whether the ray actually hit the primitive. To determine which function in the intersection table to call, Metal adds this property to the value specified in the instance’s [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/intersectionfunctiontableoffset), and looks up the entry at that index.

## See also

### Customizing intersection and hit tests for the instance
- [options](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/options) — An option set which applies to the next acceleration structure motion-instance you create with the descriptor.
- [mask](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/mask) — A mask for testing ray-tracing rays with a scene’s geometry, which applies to the next acceleration-structure motion instance you create with the descriptor.
