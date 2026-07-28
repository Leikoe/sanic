# error

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/error>

A description of an error when the GPU encounters an issue as it runs the command buffer.

## Declaration

```swift
var error: (any Error)? { get }
```

## Discussion

You typically check this property during development to get more information about a runtime issue. The property remains `nil` unless the GPU can’t successfully run the command buffer.

An error’s [userInfo](https://developer.apple.com/documentation/Foundation/NSError/userInfo) dictionary property contains additional information if the command buffer’s [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbuffer/erroroptions) property includes [encoderExecutionStatus](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption/encoderexecutionstatus). You can retrieve an [MTLCommandBufferEncoderInfo](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo) instance from the dictionary by accessing it with [MTLCommandBufferEncoderInfoErrorKey](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfoerrorkey).

## See also

### Getting error details
- [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbuffer/erroroptions) — Settings that determine which information the command buffer records about execution errors, and how it does it.
- [MTLCommandBufferEncoderInfo](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo) — A container that provides additional information about a runtime failure a GPU encounters as it runs the commands in a command buffer.
- [MTLCommandBufferEncoderInfoErrorKey](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfoerrorkey) — A key to a command buffer error’s user information dictionary that retrieves additional information about a GPU’s runtime error.
