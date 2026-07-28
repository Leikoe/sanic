# motionEndBorderMode

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionendbordermode>

Configures the motion border mode.

## Declaration

```swift
var motionEndBorderMode: MTLMotionBorderMode { get set }
```

## Discussion

This property controls what happens if Metal samples the acceleration structure after [motionEndTime](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionendtime).

Its default value is `MTLMotionBorderModeClamp`.
