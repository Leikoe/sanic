# motionTransformBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/motiontransformbuffer>

A buffer containing transformation information for instance motion keyframes, formatted according to the motion transform type.

## Declaration

```swift
var motionTransformBuffer: MTL4BufferRange { get set }
```

## Discussion

Each instance can have a different number of keyframes that you configure via individual instance descriptors.

You are responsible for ensuring the buffer address the range references is not zero when using motion instance descriptors.
