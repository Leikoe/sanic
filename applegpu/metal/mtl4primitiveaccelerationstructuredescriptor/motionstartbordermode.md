# motionStartBorderMode

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionstartbordermode>

Configures the behavior when the ray-tracing system samples the acceleration structure before the motion start time.

## Declaration

```swift
var motionStartBorderMode: MTLMotionBorderMode { get set }
```

## Discussion

Use this property to control the behavior when the ray-tracing system samples the acceleration structure at a time prior to the one you set for [motionStartTime](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionstarttime).

The default value of this property is `MTLMotionBorderModeClamp`.
