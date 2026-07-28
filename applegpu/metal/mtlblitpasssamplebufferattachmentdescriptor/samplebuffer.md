# sampleBuffer

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/samplebuffer>

A specialized memory buffer that the GPU uses to store its counter data during the blit pass.

## Declaration

```swift
var sampleBuffer: (any MTLCounterSampleBuffer)? { get set }
```

## Discussion

The property defaults to `nil`, which means the GPU doesn’t save any GPU counter information during the blit pass. For more information, see [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) and [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers).

## See also

### Configuring the sample buffer attachment
- [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/startofencodersampleindex) — An index within a counter sample buffer that tells the GPU where to store counter data from the start of a blit pass.
- [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor/endofencodersampleindex) — An index within a counter sample buffer that tells the GPU where to store counter data from the end of a blit pass.
