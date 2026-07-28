# MTLDispatchType

*Enumeration · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldispatchtype>

The type of dispatch method to use when calling encoded functions.

## Declaration

```swift
enum MTLDispatchType
```

## Topics

### Execution dispatch types
- [MTLDispatchType.concurrent](https://developer.apple.com/documentation/metal/mtldispatchtype/concurrent) — Sets a command encoder to dispatch encoded commands concurrently during your pass.
- [MTLDispatchType.serial](https://developer.apple.com/documentation/metal/mtldispatchtype/serial) — Sets a command encoder to dispatch encoded commands serially during your pass.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtldispatchtype/init(rawvalue:))

## See also

### Configuring a compute pass
- [MTLComputePassDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor) — A description of how to dispatch execution of pass commands and GPU performance sampling.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
- [MTLComputePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor) — A configuration that instructs the GPU where to store counter data from the beginning and end of a compute pass.
- [MTLComputePassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray) — A container that stores an array of sample buffer attachments for a compute pass.
