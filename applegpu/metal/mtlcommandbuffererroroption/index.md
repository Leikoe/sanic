# MTLCommandBufferErrorOption

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption>

Options for reporting errors from a command buffer.

## Declaration

```swift
struct MTLCommandBufferErrorOption
```

## Topics

### Buffer error options
- [encoderExecutionStatus](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption/encoderexecutionstatus) — An option that instructs a command buffer to save additional details about a GPU runtime error.

### Protocol support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption/init(rawvalue:)) — Creates a set of error options from a raw integer value.

## See also

### Configuring the command buffer
- [logState](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/logstate) — The shader logging configuration that the command buffer uses.
- [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/retainedreferences) — A Boolean value that indicates whether the command buffer the descriptor creates maintains strong references to the resources it uses.
- [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/erroroptions) — The reporting configuration that indicates which information the GPU driver stores in a command buffer’s error property.
