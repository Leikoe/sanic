# retainedReferences

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/retainedreferences>

A Boolean value that indicates whether the command buffer maintains strong references to the resources it uses.

## Declaration

```swift
var retainedReferences: Bool { get }
```

## Discussion

You can configure this property when you create a command buffer by setting [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/retainedreferences) of an [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) instance and calling the [makeCommandBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer(descriptor:)) method. The [makeCommandBuffer()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer()) method sets this property to [true](https://developer.apple.com/documentation/Swift/true), and [makeCommandBufferWithUnretainedReferences()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbufferwithunretainedreferences()) sets it to [false](https://developer.apple.com/documentation/Swift/false).

If [false](https://developer.apple.com/documentation/Swift/false), your app is responsible for maintaining strong references to all the resources the command buffer relies on until it completes.

> **Important:**
>  Releasing a resource before a command buffer’s commands complete may cause a runtime error or erratic behavior.
