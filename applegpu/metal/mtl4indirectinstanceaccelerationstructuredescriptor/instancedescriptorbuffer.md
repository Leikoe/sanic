# instanceDescriptorBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/instancedescriptorbuffer>

Assigns a reference to a buffer containing instance descriptors for acceleration structures to reference.

## Declaration

```swift
var instanceDescriptorBuffer: MTL4BufferRange { get set }
```

## Discussion

This buffer conceptually represents an array of instance data. The specific format for the structs that comprise each entry depends on the value of the  [instanceDescriptorType](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor/instancedescriptortype) property.

You are responsible for ensuring the buffer address the range contains is not zero.
