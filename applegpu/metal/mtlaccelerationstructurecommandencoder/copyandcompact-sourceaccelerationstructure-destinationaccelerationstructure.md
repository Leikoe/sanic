# copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)>

Encodes a command to compact an acceleration structure’s data and copy it into a different acceleration structure.

## Declaration

```swift
func copyAndCompact(sourceAccelerationStructure: any MTLAccelerationStructure, destinationAccelerationStructure: any MTLAccelerationStructure)
```

## Parameters

- **sourceAccelerationStructure** — The source acceleration structure.
- **destinationAccelerationStructure** — The destination acceleration structure.

## Discussion

The source and destination acceleration structures can’t overlap in memory. The destination acceleration structure needs to be at least as large as the compact size of the source acceleration structure, which you obtain by using the [writeCompactedSize(accelerationStructure:buffer:offset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:)) method.

If the source acceleration structure contains references to other acceleration structures, the copy of the acceleration structure refers to the same child structures.

## See also

### Copying an acceleration structure
- [copy(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes a command to copy the data from one acceleration structure to another.
- [writeCompactedSize(accelerationStructure:buffer:offset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:)) — Encodes a command to calculate the compacted size of an acceleration structure.
- [writeCompactedSize(accelerationStructure:buffer:offset:sizeDataType:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:sizedatatype:)) — Encodes a command to calculate the compacted size of an acceleration structure, taking into account the size of the output data.
