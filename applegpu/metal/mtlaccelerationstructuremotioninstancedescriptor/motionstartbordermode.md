# motionStartBorderMode

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motionstartbordermode>

A behavior that configures how a motion instance handles timestamps before a starting time.

## Declaration

```swift
var motionStartBorderMode: MTLMotionBorderMode
```

## Discussion

The property’s default value is [MTLMotionBorderMode.clamp](https://developer.apple.com/documentation/metal/mtlmotionbordermode/clamp).

## See also

### Specifying motion data
- [motionStartTime](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motionstarttime) — A starting time for the range of motion that the key-frame data represents.
- [motionEndTime](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motionendtime) — An ending time for the range of motion that the key-frame data represents.
- [motionEndBorderMode](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motionendbordermode) — A behavior that configures how a motion instance handles timestamps after an ending time.
- [motionTransformsStartIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motiontransformsstartindex) — The index of motion data that represents the first key-frame motion data, which applies to the next acceleration-structure motion instance you create with the descriptor.
- [motionTransformsCount](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motiontransformscount) — The number of motion data key-frames, which applies to the next acceleration-structure motion instance you create with the descriptor.
