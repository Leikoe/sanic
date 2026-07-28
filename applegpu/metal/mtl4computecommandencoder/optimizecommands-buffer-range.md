# optimizeCommands(buffer:range:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecommands(buffer:range:)>

Encode a command to attempt to improve the performance of a range of commands within an indirect command buffer.

## Declaration

```swift
func optimizeCommands(buffer: any MTLIndirectCommandBuffer, range: Range<Int>)
```

## Parameters

- **buffer** — An [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance that this command optimizes.
- **range** — A range of commands within `indirectCommandBuffer`.

## See also

### Encoding optimization commands
- [optimizeContents(forCPUAccess:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forcpuaccess:)) — Encodes a command that modifies the contents of a texture to improve the performance of CPU accesses to its contents.
- [optimizeContents(forCPUAccess:slice:level:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forcpuaccess:slice:level:)) — Encodes a command that modifies the contents of a texture to improve the performance of CPU accesses to its contents in a specific region.
- [optimizeContents(forGPUAccess:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forgpuaccess:)) — Encodes a command that modifies the contents of a texture to improve the performance of GPU accesses to its contents.
- [optimizeContents(forGPUAccess:slice:level:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forgpuaccess:slice:level:)) — Encodes a command that modifies the contents of a texture instance to improve the performance of GPU accesses to its contents in a specific region.
