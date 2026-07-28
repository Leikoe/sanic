# motionTransformCountBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/motiontransformcountbuffer>

Associates a buffer reference containing the number of motion transforms in the motion transform buffer, formatted as a 32-bit unsigned integer.

## Declaration

```swift
var motionTransformCountBuffer: MTL4BufferRange { get set }
```

## Discussion

You are responsible for ensuring that the final number of motion transforms at build time in the buffer this property references is less than or equal to the value of property [maxMotionTransformCount](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/maxmotiontransformcount).
