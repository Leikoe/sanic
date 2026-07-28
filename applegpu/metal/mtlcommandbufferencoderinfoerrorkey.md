# MTLCommandBufferEncoderInfoErrorKey

*Global Variable · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfoerrorkey>

A key to a command buffer error’s user information dictionary that retrieves additional information about a GPU’s runtime error.

## Declaration

```swift
let MTLCommandBufferEncoderInfoErrorKey: String
```

## Discussion

You can retrieve an [MTLCommandBufferEncoderInfo](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo) instance from the [userInfo](https://developer.apple.com/documentation/Foundation/NSError/userInfo) dictionary of a command buffer’s [error](https://developer.apple.com/documentation/metal/mtlcommandbuffer/error) property.

## See also

### Getting error details
- [error](https://developer.apple.com/documentation/metal/mtlcommandbuffer/error) — A description of an error when the GPU encounters an issue as it runs the command buffer.
- [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbuffer/erroroptions) — Settings that determine which information the command buffer records about execution errors, and how it does it.
- [MTLCommandBufferEncoderInfo](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo) — A container that provides additional information about a runtime failure a GPU encounters as it runs the commands in a command buffer.
