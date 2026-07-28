# MTLCommandBufferEncoderInfo

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo>

A container that provides additional information about a runtime failure a GPU encounters as it runs the commands in a command buffer.

## Declaration

```swift
protocol MTLCommandBufferEncoderInfo : NSObjectProtocol
```

## Overview

To create a command buffer that generates additional information (when a GPU encounters an error running it), configure an [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) instance’s [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/erroroptions) property. For information about how to retrieve the information from an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance, see its [error](https://developer.apple.com/documentation/metal/mtlcommandbuffer/error) property.

## Topics

### Inspecting execution information
- [label](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/label) — The name of the encoder that generates the error information.
- [debugSignposts](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/debugsignposts) — An array of debug signposts that Metal records as the GPU executes the commands of the encoder’s pass.
- [errorState](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo/errorstate) — The execution status of the command encoder.
- [MTLCommandEncoderErrorState](https://developer.apple.com/documentation/metal/mtlcommandencodererrorstate) — Possible error conditions for the command encoder’s commands.

## See also

### Getting error details
- [error](https://developer.apple.com/documentation/metal/mtlcommandbuffer/error) — A description of an error when the GPU encounters an issue as it runs the command buffer.
- [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbuffer/erroroptions) — Settings that determine which information the command buffer records about execution errors, and how it does it.
- [MTLCommandBufferEncoderInfoErrorKey](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfoerrorkey) — A key to a command buffer error’s user information dictionary that retrieves additional information about a GPU’s runtime error.
