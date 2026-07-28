# MTLAccelerationStructureCommandEncoder

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder>

Encodes commands that build and refit acceleration structures for a single pass.

## Declaration

```swift
protocol MTLAccelerationStructureCommandEncoder : MTLCommandEncoder
```

## Overview

Create an acceleration structure encoder by calling one of the factory methods on an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance, such as [makeAccelerationStructureCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeaccelerationstructurecommandencoder()).

### Command stages

Most commands apply to one stage within a pass. The following table shows which stage applies to each command:

| Function | MTLStages |
|---|---|
| [build(accelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/build(accelerationstructure:descriptor:scratchbuffer:scratchbufferoffset:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [copy(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [writeCompactedSize(accelerationStructure:buffer:offset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [writeCompactedSize(accelerationStructure:buffer:offset:sizeDataType:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:sizedatatype:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [refit(sourceAccelerationStructure:descriptor:destinationAccelerationStructure:scratchBuffer:scratchBufferOffset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/refit(sourceaccelerationstructure:descriptor:destinationaccelerationstructure:scratchbuffer:scratchbufferoffset:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [refit(sourceAccelerationStructure:descriptor:destinationAccelerationStructure:scratchBuffer:scratchBufferOffset:options:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/refit(sourceaccelerationstructure:descriptor:destinationaccelerationstructure:scratchbuffer:scratchbufferoffset:options:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) | None |

For more information about stages and synchronization, see [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) and [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

## Topics

### Building an acceleration structure
- [build(accelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/build(accelerationstructure:descriptor:scratchbuffer:scratchbufferoffset:)) — Encodes a command to build a new acceleration structure.

### Copying an acceleration structure
- [copy(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes a command to copy the data from one acceleration structure to another.
- [writeCompactedSize(accelerationStructure:buffer:offset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:)) — Encodes a command to calculate the compacted size of an acceleration structure.
- [writeCompactedSize(accelerationStructure:buffer:offset:sizeDataType:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/writecompactedsize(accelerationstructure:buffer:offset:sizedatatype:)) — Encodes a command to calculate the compacted size of an acceleration structure, taking into account the size of the output data.
- [copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes a command to compact an acceleration structure’s data and copy it into a different acceleration structure.

### Refitting an acceleration structure
- [refit(sourceAccelerationStructure:descriptor:destinationAccelerationStructure:scratchBuffer:scratchBufferOffset:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/refit(sourceaccelerationstructure:descriptor:destinationaccelerationstructure:scratchbuffer:scratchbufferoffset:)) — Updates an acceleration structure with new geometry or instance data.
- [refit(sourceAccelerationStructure:descriptor:destinationAccelerationStructure:scratchBuffer:scratchBufferOffset:options:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/refit(sourceaccelerationstructure:descriptor:destinationaccelerationstructure:scratchbuffer:scratchbufferoffset:options:)) — Updates an acceleration structure with new geometry or instance data, with options that control the refitting process.

### Preventing resource access conflicts
- [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/updatefence(_:)) — Encodes a command that instructs the GPU to update a fence after the acceleration structure pass completes.
- [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/waitforfence(_:)) — Encodes a command that instructs the GPU to pause the acceleration structure pass until another pass updates a fence.

### Making indirect resources resident
- [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useheap(_:)) — Makes the resources contained in the specified heap available to the acceleration structure pass.
- [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useheaps(_:)) — Makes the resources contained in the specified heaps available to the acceleration structure pass.
- [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useresource(_:usage:)) — Makes a resource available to the acceleration structure pass.
- [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useresources(_:usage:)) — Makes multiple resources available to the acceleration structure pass.
- [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) — Options that describe how a graphics or compute function uses an argument buffer’s resource.

### Sampling counters
- [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) — Encodes a command to sample hardware counters at this point in the acceleration structure pass and store the samples into a counter sample buffer.

## See also

### Acceleration structures
- [Improving ray-tracing data access using per-primitive data](https://developer.apple.com/documentation/metal/improving-ray-tracing-data-access-using-per-primitive-data) — Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
- [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) — A collection of model data for GPU-accelerated intersection of rays with the model.
- [MTL4AccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor) — Base class for Metal 4 acceleration structure descriptors.
- [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) — A base class for classes that define the configuration for a new acceleration structure.
- [MTL4PrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor) — Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.
- [MTLPrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor) — A description of an acceleration structure that contains geometry primitives.
- [MTL4InstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor) — Descriptor for an instance acceleration structure.
- [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that derives from instances of primitive acceleration structures.
- [MTLAccelerationStructureUsage](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage) — Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.
- [MTLAccelerationStructureRefitOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructurerefitoptions)
