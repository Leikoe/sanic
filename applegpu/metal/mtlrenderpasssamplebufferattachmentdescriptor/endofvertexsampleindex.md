# endOfVertexSampleIndex

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/endofvertexsampleindex>

The index the Metal device object should use to store GPU counters when ending the render pass’s vertex stage.

## Declaration

```swift
var endOfVertexSampleIndex: Int { get set }
```

## Discussion

Specify [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample) if you don’t want to sample GPU counters at the end of the vertex stage. Otherwise, specify an index within the sample buffer where you want the GPU to write the sample data.

On devices that don’t support [MTLCounterSamplingPoint.atStageBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atstageboundary) you need to set the value to [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample).

## See also

### Configuring the sample buffer attachment
- [sampleBuffer](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/samplebuffer) — A specialized memory buffer that the GPU uses to store its counter data during the render pass.
- [startOfVertexSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/startofvertexsampleindex) — The index the Metal device object should use to store GPU counters when starting the render pass’s vertex stage.
- [startOfFragmentSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/startoffragmentsampleindex) — The index the Metal device object should use to store GPU counters when starting the render pass’s fragment stage.
- [endOfFragmentSampleIndex](https://developer.apple.com/documentation/metal/mtlrenderpasssamplebufferattachmentdescriptor/endoffragmentsampleindex) — The index the Metal device object should use to store GPU counters when ending the render pass’s fragment stage.
