# copy(sourceAccelerationStructure:destinationAccelerationStructure:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)>

Encodes a command to copy the data from one acceleration structure to another.

## Declaration

```swift
func copy(sourceAccelerationStructure: any MTLAccelerationStructure, destinationAccelerationStructure: any MTLAccelerationStructure)
```

## Parameters

- **sourceAccelerationStructure** — The source acceleration structure.
- **destinationAccelerationStructure** — The destination acceleration structure.

## Discussion

The destination acceleration structure needs to be at least as large as the source acceleration structure, unless you’re compacting the source acceleration structure. In that case, the destination acceleration structure needs be at least as large as the compact size of the source acceleration structure.

If the source acceleration structure contains references to other acceleration structures, the copy of the acceleration structure also refers to the same child structures.

## See also

### Copying an acceleration structure
- [writeCompactedSize(accelerationStructure:buffer:offset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:)) — Encodes a command to calculate the compacted size of an acceleration structure.
- [writeCompactedSize(accelerationStructure:buffer:offset:sizeDataType:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:sizedatatype:)) — Encodes a command to calculate the compacted size of an acceleration structure, taking into account the size of the output data.
- [copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes a command to compact an acceleration structure’s data and copy it into a different acceleration structure.
