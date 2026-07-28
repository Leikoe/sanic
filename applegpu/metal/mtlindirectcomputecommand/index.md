# MTLIndirectComputeCommand

*Protocol · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 11.0, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcomputecommand>

A compute command in an indirect command buffer.

## Declaration

```swift
protocol MTLIndirectComputeCommand : NSObjectProtocol
```

## Overview

Don’t implement this protocol; you get instances of this type by asking an [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) for them.

Use this instance to reset or encode a command. You need to reset a command before encoding a new command.

## Topics

### Setting a command’s arguments
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setcomputepipelinestate(_:)) — Sets the command’s compute pipeline state.
- [setImageblockWidth(_:height:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setimageblockwidth(_:height:)) — Sets the size, in pixels, of the imageblock.
- [setKernelBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setkernelbuffer(_:offset:at:)) — Sets a buffer for the compute function.
- [setThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:index:)) — Sets the size of a block of threadgroup memory.
- [setThreadgroupMemoryLength(_:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:at:)) — Sets the size of a block of threadgroup memory.
- [setStageInRegion(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstageinregion(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.
- [setStageIn(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstagein(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.

### Synchronizing command execution
- [setBarrier()](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setbarrier()) — Adds a barrier to ensure that commands executed prior to this command are complete before this command executes.
- [clearBarrier()](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/clearbarrier()) — Removes any barrier set on the command.

### Encoding a compute command
- [concurrentDispatchThreadgroups(_:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/concurrentdispatchthreadgroups(_:threadsperthreadgroup:)) — Encodes a compute command using a grid aligned to threadgroup boundaries.
- [concurrentDispatchThreads(_:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/concurrentdispatchthreads(_:threadsperthreadgroup:)) — Encodes a compute command using an arbitrarily sized grid.

### Resetting a command
- [reset()](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/reset()) — Resets the command to its default state.

### Instance Methods
- [setKernelBuffer(_:offset:attributeStride:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setkernelbuffer(_:offset:attributestride:at:))

## See also

### Indirect compute commands
- [MTLRegion](https://developer.apple.com/documentation/metal/mtlregion) — The bounds for a subset of an instance’s elements.
- [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) — A type that represents one, two, or three dimensions of a type instance, such as an array or texture.
- [MTLOrigin](https://developer.apple.com/documentation/metal/mtlorigin) — The coordinates for the front upper-left corner of a region.
- [MTLStageInRegionIndirectArguments](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments) — The data layout required for the arguments needed to specify the stage-in region.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
