# copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)>

Encodes a command to copy and compact an acceleration structure.

## Declaration

```swift
func copyAndCompact(sourceAccelerationStructure: any MTLAccelerationStructure, destinationAccelerationStructure: any MTLAccelerationStructure)
```

## Parameters

- **sourceAccelerationStructure** — Acceleration structure to copy and compact.
- **destinationAccelerationStructure** — Acceleration structure to copy to.

## Discussion

You are responsible for ensuring that the source and destination acceleration structures don’t overlap in memory. If this is an instance acceleration structure, Metal preserves references to primitive acceleration structures it references.

This operation requires that the destination acceleration structure is at least as large as the compacted size of the source acceleration structure. You can compute this size by calling the [writeCompactedSize(sourceAccelerationStructure:destinationBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writecompactedsize(sourceaccelerationstructure:destinationbuffer:)) method.

## See also

### Encoding acceleration structure copy commands
- [copy(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes an acceleration structure copy operation into the command buffer.
- [writeCompactedSize(sourceAccelerationStructure:destinationBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writecompactedsize(sourceaccelerationstructure:destinationbuffer:)) — Encodes a command to compute the size an acceleration structure can compact into, writing the result into a buffer.
