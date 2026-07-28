# motionTransformType

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/motiontransformtype>

Sets the type of motion transforms, either as a matrix or individual components.

## Declaration

```swift
var motionTransformType: MTLTransformType { get set }
```

## Discussion

Defaults to `MTLTransformTypePackedFloat4x3`. Using a `MTLTransformTypeComponent` allows you to represent the rotation by a quaternion (instead as of part of the matrix), allowing for correct motion interpolation.
