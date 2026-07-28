# MTLMotionBorderMode

*Enumeration · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlmotionbordermode>

Options for specifying how the acceleration structure handles timestamps that are outside the specified range.

## Declaration

```swift
enum MTLMotionBorderMode
```

## Overview

The [motionStartBorderMode](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionstartbordermode) and [motionEndBorderMode](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionendbordermode) properties use this type to describe the behavior for a motion-based object when a timestamp is outside the specified range.

## Topics

### Specifying motion modes
- [MTLMotionBorderMode.clamp](https://developer.apple.com/documentation/metal/mtlmotionbordermode/clamp) — A mode that specifies treating times outside the specified endpoint as if they were at the endpoint.
- [MTLMotionBorderMode.vanish](https://developer.apple.com/documentation/metal/mtlmotionbordermode/vanish) — A mode that specifies that times outside the specified endpoint need to prevent any ray-intersections with the primitive.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlmotionbordermode/init(rawvalue:))

## See also

### Specifying motion behavior
- [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionkeyframecount) — The number of keyframes in the geometry data.
- [motionStartTime](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionstarttime) — The start time for the range of motion that the keyframe data describes.
- [motionEndTime](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionendtime) — The end time for the range of motion that the keyframe data describes.
- [motionStartBorderMode](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionstartbordermode) — The mode to use when handling timestamps before the start time.
- [motionEndBorderMode](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionendbordermode) — The mode to use when handling timestamps after the end time.
