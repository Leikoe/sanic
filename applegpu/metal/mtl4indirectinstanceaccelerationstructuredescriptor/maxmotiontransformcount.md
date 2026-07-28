# maxMotionTransformCount

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/maxmotiontransformcount>

Controls the maximum number of motion transforms in the motion transform buffer.

## Declaration

```swift
var maxMotionTransformCount: Int { get set }
```

## Discussion

You are responsible for ensuring that final number of motion transforms at build time that the buffer [motionTransformCountBuffer](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/motiontransformcountbuffer) references is less than or equal to this number.
