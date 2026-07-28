# MTLComputePassSampleBufferAttachmentDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor>

A configuration that instructs the GPU where to store counter data from the beginning and end of a compute pass.

## Declaration

```swift
class MTLComputePassSampleBufferAttachmentDescriptor
```

## Overview

For more context about configuring sample buffer attachments for compute passes, see [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers). That article is one of a series in [GPU counters and counter sample buffers](https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers) about sampling Metal hardware counters for performance measurement.

## Topics

### Configuring the sample buffer attachment
- [sampleBuffer](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/samplebuffer) — A specialized memory buffer that the GPU uses to store its counter data during a compute pass.
- [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/startofencodersampleindex) — An index within a counter sample buffer that tells the GPU where to store counter data from the start of a compute pass.
- [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/endofencodersampleindex) — An index within a counter sample buffer that tells the GPU where to store counter data from the end of a compute pass.

## See also

### Configuring a compute pass
- [MTLComputePassDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor) — A description of how to dispatch execution of pass commands and GPU performance sampling.
- [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) — The type of dispatch method to use when calling encoded functions.
- [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) — The data layout required for arguments needed to specify the size of threadgroups.
- [MTLComputePassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray) — A container that stores an array of sample buffer attachments for a compute pass.
