# resetCommandsInBuffer(_:range:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 12.0, macOS 10.14, tvOS 12.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resetcommandsinbuffer(_:range:)>

Encodes a command that resets a range of commands in an indirect command buffer.

## Declaration

```swift
func resetCommandsInBuffer(_ buffer: any MTLIndirectCommandBuffer, range: Range<Int>)
```

## Parameters

- **buffer** — An indirect command buffer the command resets.
- **range** — A range of commands within `buffer`.

## See also

### Managing indirect command buffers
- [copyIndirectCommandBuffer(_:sourceRange:destination:destinationIndex:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copyindirectcommandbuffer(_:sourcerange:destination:destinationindex:)) — Encodes a command that copies commands from one indirect command buffer into another.
- [optimizeIndirectCommandBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizeindirectcommandbuffer(_:range:)) — Encodes a command that can improve the performance of a range of commands within an indirect command buffer.
