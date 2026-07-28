# copy(sourceAccelerationStructure:destinationAccelerationStructure:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)>

Encodes an acceleration structure copy operation into the command buffer.

## Declaration

```swift
func copy(sourceAccelerationStructure: any MTLAccelerationStructure, destinationAccelerationStructure: any MTLAccelerationStructure)
```

## Parameters

- **sourceAccelerationStructure** — Acceleration structure to copy from.
- **destinationAccelerationStructure** — Acceleration structure to copy to.

## Discussion

You are responsible for ensuring the source and destination acceleration structures don’t overlap in memory. If this is an instance acceleration structure, Metal preserves references to the primitive acceleration structures it references.

Typically, the destination acceleration structure is at least as large as the source acceleration structure, except in cases where you compact the source acceleration structure. In this case, you need to allocate the destination acceleration to be at least as large as the compacted size of the source acceleration structure.

## See also

### Encoding acceleration structure copy commands
- [copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes a command to copy and compact an acceleration structure.
- [writeCompactedSize(sourceAccelerationStructure:destinationBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writecompactedsize(sourceaccelerationstructure:destinationbuffer:)) — Encodes a command to compute the size an acceleration structure can compact into, writing the result into a buffer.
