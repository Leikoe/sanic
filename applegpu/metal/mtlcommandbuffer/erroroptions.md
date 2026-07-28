# errorOptions

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/erroroptions>

Settings that determine which information the command buffer records about execution errors, and how it does it.

## Declaration

```swift
var errorOptions: MTLCommandBufferErrorOption { get }
```

## Discussion

The property reflects the [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/erroroptions) property of the [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) instance at the time you create the command buffer.

## See also

### Getting error details
- [error](https://developer.apple.com/documentation/metal/mtlcommandbuffer/error) — A description of an error when the GPU encounters an issue as it runs the command buffer.
- [MTLCommandBufferEncoderInfo](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo) — A container that provides additional information about a runtime failure a GPU encounters as it runs the commands in a command buffer.
- [MTLCommandBufferEncoderInfoErrorKey](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfoerrorkey) — A key to a command buffer error’s user information dictionary that retrieves additional information about a GPU’s runtime error.
