# sampleBufferAttachments

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor/samplebufferattachments>

The sample buffers that the compute pass can access.

## Declaration

```swift
var sampleBufferAttachments: MTLComputePassSampleBufferAttachmentDescriptorArray { get }
```

## Discussion

The GPU uses sample buffers to record performance information. See [GPU counters and counter sample buffers](https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers), [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers), and [MTLCounter](https://developer.apple.com/documentation/metal/mtlcounter) for more information.
