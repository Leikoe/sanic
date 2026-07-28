# MTLBlitPassSampleBufferAttachmentDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor>

A configuration that instructs the GPU where to store counter data from the beginning and end of a blit pass.

## Declaration

```swift
class MTLBlitPassSampleBufferAttachmentDescriptor
```

## Overview

See [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers) for more context about configuring instances of this type. That article is one of a series of articles in [GPU counters and counter sample buffers](https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers).

## Topics

### Configuring the sample buffer attachment
- [sampleBuffer](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/samplebuffer) — A specialized memory buffer that the GPU uses to store its counter data during the blit pass.
- [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/startofencodersampleindex) — An index within a counter sample buffer that tells the GPU where to store counter data from the start of a blit pass.
- [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/endofencodersampleindex) — An index within a counter sample buffer that tells the GPU where to store counter data from the end of a blit pass.

## See also

### Configuring a blit command encoder
- [MTLBlitPassDescriptor](https://developer.apple.com/documentation/metal/mtlblitpassdescriptor) — A configuration you create to customize a blit command encoder, which affects the runtime behavior of the blit pass you encode with it.
- [MTLBlitPassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptorarray) — A container that stores an array of sample buffer attachments for a blit pass.
