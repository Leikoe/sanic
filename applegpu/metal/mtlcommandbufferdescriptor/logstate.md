# logState

*Instance Property · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/logstate>

The shader logging configuration that the command buffer uses.

## Declaration

```swift
var logState: (any MTLLogState)? { get set }
```

## See also

### Configuring the command buffer
- [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/retainedreferences) — A Boolean value that indicates whether the command buffer the descriptor creates maintains strong references to the resources it uses.
- [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/erroroptions) — The reporting configuration that indicates which information the GPU driver stores in a command buffer’s error property.
- [MTLCommandBufferErrorOption](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption) — Options for reporting errors from a command buffer.
