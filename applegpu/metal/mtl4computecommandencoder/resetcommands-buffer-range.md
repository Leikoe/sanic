# resetCommands(buffer:range:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/resetcommands(buffer:range:)>

Encodes a command that resets a range of commands in an indirect command buffer.

## Declaration

```swift
func resetCommands(buffer: any MTLIndirectCommandBuffer, range: Range<Int>)
```

## Parameters

- **buffer** — An [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) the command resets.
- **range** — A range of commands within `buffer`.
