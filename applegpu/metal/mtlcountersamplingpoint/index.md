# MTLCounterSamplingPoint

*Enumeration · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplingpoint>

Options for different times when you can sample GPU counters.

## Declaration

```swift
enum MTLCounterSamplingPoint
```

## Topics

### Reading sampling boundary types
- [MTLCounterSamplingPoint.atBlitBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atblitboundary) — Counter sampling is allowed between blit commands in a blit pass.
- [MTLCounterSamplingPoint.atDispatchBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atdispatchboundary) — Counter sampling is allowed between kernel dispatches in a compute pass.
- [MTLCounterSamplingPoint.atDrawBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atdrawboundary) — Counter sampling is allowed between draw commands in a render pass.
- [MTLCounterSamplingPoint.atStageBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atstageboundary) — Counter sampling is allowed at the start and end of a render pass’s vertex and fragment stages, and at the start and end of compute and blit passes.
- [MTLCounterSamplingPoint.atTileDispatchBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/attiledispatchboundary) — Counter sampling is allowed between tile dispatches in a render pass.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/init(rawvalue:))

## See also

### Sampling a GPU device’s counters
- [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) — The counter sets supported by the device object.
- [supportsCounterSampling(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportscountersampling(_:)) — Returns a Boolean value that indicates whether you can read GPU counters at the specified command boundary.
- [makeCounterSampleBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecountersamplebuffer(descriptor:)) — Creates a counter sample buffer.
