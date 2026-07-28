# sampleBuffer

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/samplebuffer>

A specialized memory buffer that the GPU uses to store its counter data during the resource state pass.

## Declaration

```swift
var sampleBuffer: (any MTLCounterSampleBuffer)? { get set }
```

## Discussion

The property defaults to `nil`, which means the GPU doesn’t save any GPU counter information during the resource state pass. For more information, see [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) and [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers).

## See also

### Configuring the sample buffer attachment
- [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/startofencodersampleindex) — The index the Metal device object should use to store GPU counters when starting the resource state pass.
- [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/endofencodersampleindex) — The index the Metal device object should use to store GPU counters when ending the resource state pass.
