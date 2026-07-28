# writeCompactedSize(sourceAccelerationStructure:destinationBuffer:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writecompactedsize(sourceaccelerationstructure:destinationbuffer:)>

Encodes a command to compute the size an acceleration structure can compact into, writing the result into a buffer.

## Declaration

```swift
func writeCompactedSize(sourceAccelerationStructure accelerationStructure: any MTLAccelerationStructure, destinationBuffer buffer: MTL4BufferRange)
```

## Parameters

- **accelerationStructure** — Source acceleration structure.
- **buffer** — Destination size buffer. Metal writes the compacted size as a 64-bit unsigned integer value, representing the compacted size in bytes.

## Discussion

This size is potentially smaller than the acceleration structure. To perform compaction, you typically read this size from the buffer once the command buffer completes. You then use it to allocate a new, potentially smaller acceleration structure. Finally, you call the [copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)) method to perform the copy.

## See also

### Encoding acceleration structure copy commands
- [copy(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes an acceleration structure copy operation into the command buffer.
- [copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes a command to copy and compact an acceleration structure.
