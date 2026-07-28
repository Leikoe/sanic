# maxInstanceCount

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/maxinstancecount>

Controls the maximum number of instance descriptors the instance descriptor buffer can reference.

## Declaration

```swift
var maxInstanceCount: Int { get set }
```

## Discussion

You are responsible for ensuring that the final number of instances at build time, which you provide indirectly via a buffer reference in [instanceCountBuffer](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/instancecountbuffer), is less than or equal to this number.
