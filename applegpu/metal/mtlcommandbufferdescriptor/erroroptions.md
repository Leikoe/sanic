# errorOptions

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/erroroptions>

The reporting configuration that indicates which information the GPU driver stores in a command buffer’s error property.

## Declaration

```swift
var errorOptions: MTLCommandBufferErrorOption { get set }
```

## Discussion

By default, a GPU driver doesn’t report additional error information.

To create a command buffer that saves additional GPU runtime error information, add the [encoderExecutionStatus](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption/encoderexecutionstatus) option to this property. If the GPU encounters an error as it runs the command buffer, you can retrieve the additional information from the command buffer’s [error](https://developer.apple.com/documentation/metal/mtlcommandbuffer/error) property.

> **Note:**
>  Enabling the [encoderExecutionStatus](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption/encoderexecutionstatus) option can slightly reduce your app’s CPU runtime performance.

## See also

### Configuring the command buffer
- [logState](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/logstate) — The shader logging configuration that the command buffer uses.
- [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/retainedreferences) — A Boolean value that indicates whether the command buffer the descriptor creates maintains strong references to the resources it uses.
- [MTLCommandBufferErrorOption](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption) — Options for reporting errors from a command buffer.
