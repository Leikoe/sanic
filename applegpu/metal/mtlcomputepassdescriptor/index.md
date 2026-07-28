# MTLComputePassDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor>

A description of how to dispatch execution of pass commands and GPU performance sampling.

## Declaration

```swift
class MTLComputePassDescriptor
```

## Topics

### Configuring the dispatch mechanism
- [dispatchType](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor/dispatchtype) — The strategy for dispatching any compute commands encoded in the compute pass.

### Specifying sample buffers for GPU counters
- [sampleBufferAttachments](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor/samplebufferattachments) — The sample buffers that the compute pass can access.

## See also

### Configuring a compute pass
- [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) — The type of dispatch method to use when calling encoded functions.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
- [MTLComputePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor) — A configuration that instructs the GPU where to store counter data from the beginning and end of a compute pass.
- [MTLComputePassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray) — A container that stores an array of sample buffer attachments for a compute pass.
