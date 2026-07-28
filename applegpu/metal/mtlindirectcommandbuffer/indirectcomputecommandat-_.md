# indirectComputeCommandAt(_:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectcomputecommandat(_:)>

Gets the compute command at the given index.

## Declaration

```swift
func indirectComputeCommandAt(_ commandIndex: Int) -> any MTLIndirectComputeCommand
```

## Parameters

- **commandIndex** — The index of the command to retrieve.

## Discussion

Call this method only if the indirect command buffer contains compute commands.

## See also

### Retrieving commands
- [indirectRenderCommandAt(_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectrendercommandat(_:)) — Gets the render command at the given index.
- [indirectComputeCommand(at:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectcomputecommand(at:)) — Gets the compute command at the given index.
