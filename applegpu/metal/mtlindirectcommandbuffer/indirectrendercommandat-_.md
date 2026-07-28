# indirectRenderCommandAt(_:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectrendercommandat(_:)>

Gets the render command at the given index.

## Declaration

```swift
func indirectRenderCommandAt(_ commandIndex: Int) -> any MTLIndirectRenderCommand
```

## Parameters

- **commandIndex** — The index of the command to retrieve.

## Discussion

Call this method only if the indirect command buffer contains rendering commands.

## See also

### Retrieving commands
- [indirectComputeCommandAt(_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectcomputecommandat(_:)) — Gets the compute command at the given index.
- [indirectComputeCommand(at:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectcomputecommand(at:)) — Gets the compute command at the given index.
