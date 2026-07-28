# MTLComputePassSampleBufferAttachmentDescriptorArray

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray>

A container that stores an array of sample buffer attachments for a compute pass.

## Declaration

```swift
class MTLComputePassSampleBufferAttachmentDescriptorArray
```

## Overview

The number of elements in the array is at least the number of elements in an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) property.

## Topics

### Accessing a sample buffer attachment
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray/subscript(_:)) — Returns the descriptor object for the specified sample buffer attachment.

## See also

### Configuring a compute pass
- [MTLComputePassDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor) — A description of how to dispatch execution of pass commands and GPU performance sampling.
- [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) — The type of dispatch method to use when calling encoded functions.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
- [MTLComputePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor) — A configuration that instructs the GPU where to store counter data from the beginning and end of a compute pass.
