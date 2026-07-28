# sampleBuffer

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/samplebuffer>

A specialized memory buffer that the GPU uses to store its counter data during a compute pass.

## Declaration

```swift
var sampleBuffer: (any MTLCounterSampleBuffer)? { get set }
```

## Discussion

The property defaults to `nil`, which means the GPU doesn’t save any GPU counter information during the compute pass. For more information, see [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) and [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers).

## See also

### Configuring the sample buffer attachment
- [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/startofencodersampleindex) — An index within a counter sample buffer that tells the GPU where to store counter data from the start of a compute pass.
- [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor/endofencodersampleindex) — An index within a counter sample buffer that tells the GPU where to store counter data from the end of a compute pass.
