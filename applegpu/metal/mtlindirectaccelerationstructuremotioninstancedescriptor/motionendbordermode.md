# motionEndBorderMode

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motionendbordermode>

The motion border mode describing what happens if Metal samples the acceleration structure after the motion end time.

## Declaration

```swift
var motionEndBorderMode: MTLMotionBorderMode
```

## See also

### Specifying motion data
- [motionStartTime](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motionstarttime) — The start time of the motion instance.
- [motionStartBorderMode](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motionstartbordermode) — The motion border mode describing what happens if Metal samples the acceleration structure before the motion start time.
- [motionEndTime](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motionendtime) — The end time of the motion instance.
- [motionTransformsCount](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motiontransformscount) — The number of motion transforms belonging to the motion instance.
- [motionTransformsStartIndex](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motiontransformsstartindex) — The index of the first set of transforms describing one keyframe of the animation.
