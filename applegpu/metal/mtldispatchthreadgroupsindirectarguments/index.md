# MTLDispatchThreadgroupsIndirectArguments

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments>

The data layout required for arguments needed to specify the size of threadgroups.

## Declaration

```swift
struct MTLDispatchThreadgroupsIndirectArguments
```

## Topics

### Specifying the size of the threadgroup
- [init()](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments/init()) — Returns a new data layout for dispatching threadgroups over indirect buffer calls.
- [init(threadgroupsPerGrid:)](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments/init(threadgroupspergrid:)) — Returns a new data layout for dispatching threadgroups over indirect buffer calls, with specified threadgroups per grid.
- [threadgroupsPerGrid](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments/threadgroupspergrid) — The number of threadgroups for the grid, in each dimension.

## See also

### Related Documentation
- [dispatchThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(indirectbuffer:indirectbufferoffset:threadsperthreadgroup:)) — Encodes a dispatch call for a compute pass, using an indirect buffer that defines the size of a grid that aligns to threadgroup boundaries.

### Configuring a compute pass
- [MTLComputePassDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor) — A description of how to dispatch execution of pass commands and GPU performance sampling.
- [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) — The type of dispatch method to use when calling encoded functions.
- [MTLComputePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor) — A configuration that instructs the GPU where to store counter data from the beginning and end of a compute pass.
- [MTLComputePassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray) — A container that stores an array of sample buffer attachments for a compute pass.
