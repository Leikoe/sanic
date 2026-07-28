# endOfEncoderSampleIndex

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/endofencodersampleindex>

The index the Metal device object should use to store GPU counters when ending the resource state pass.

## Declaration

```swift
var endOfEncoderSampleIndex: Int { get set }
```

## Discussion

Specify [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample) if you don’t want to sample GPU counters at the end of the resource state pass. Otherwise, specify an index within the sample buffer where you want the GPU to write the sample data.

On devices that don’t support [MTLCounterSamplingPoint.atStageBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atstageboundary) you need to set the value to [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample).

## See also

### Configuring the sample buffer attachment
- [sampleBuffer](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/samplebuffer) — A specialized memory buffer that the GPU uses to store its counter data during the resource state pass.
- [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/startofencodersampleindex) — The index the Metal device object should use to store GPU counters when starting the resource state pass.
