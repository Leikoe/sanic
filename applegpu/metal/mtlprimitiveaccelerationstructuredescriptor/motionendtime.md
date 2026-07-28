# motionEndTime

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionendtime>

The end time for the range of motion that the keyframe data describes.

## Declaration

```swift
var motionEndTime: Float { get set }
```

## Discussion

The default value is `1.0f`.

## See also

### Specifying motion behavior
- [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionkeyframecount) — The number of keyframes in the geometry data.
- [motionStartTime](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionstarttime) — The start time for the range of motion that the keyframe data describes.
- [motionStartBorderMode](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionstartbordermode) — The mode to use when handling timestamps before the start time.
- [motionEndBorderMode](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionendbordermode) — The mode to use when handling timestamps after the end time.
- [MTLMotionBorderMode](https://developer.apple.com/documentation/metal/mtlmotionbordermode) — Options for specifying how the acceleration structure handles timestamps that are outside the specified range.
