# sampleBuffer

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/samplebuffer>

A specialized memory buffer that the GPU uses to store its counter data during the render pass.

## Declaration

```swift
var sampleBuffer: (any MTLCounterSampleBuffer)? { get set }
```

## Discussion

The property defaults to `nil`, which means the GPU doesn’t save any GPU counter information during the render pass. For more information, see [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) and [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers).

## See also

### Configuring the sample buffer attachment
- [startOfVertexSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/startofvertexsampleindex) — The index the Metal device object should use to store GPU counters when starting the render pass’s vertex stage.
- [endOfVertexSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/endofvertexsampleindex) — The index the Metal device object should use to store GPU counters when ending the render pass’s vertex stage.
- [startOfFragmentSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/startoffragmentsampleindex) — The index the Metal device object should use to store GPU counters when starting the render pass’s fragment stage.
- [endOfFragmentSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/endoffragmentsampleindex) — The index the Metal device object should use to store GPU counters when ending the render pass’s fragment stage.
