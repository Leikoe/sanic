# MTLCounterSamplingPoint.atBlitBoundary

*Case · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atblitboundary>

Counter sampling is allowed between blit commands in a blit pass.

## Declaration

```swift
case atBlitBoundary
```

## Discussion

When a Metal device instance supports this sampling boundary, you can call the [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) method on an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) to sample the counters between individual blit commands.

## See also

### Reading sampling boundary types
- [MTLCounterSamplingPoint.atDispatchBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atdispatchboundary) — Counter sampling is allowed between kernel dispatches in a compute pass.
- [MTLCounterSamplingPoint.atDrawBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atdrawboundary) — Counter sampling is allowed between draw commands in a render pass.
- [MTLCounterSamplingPoint.atStageBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/atstageboundary) — Counter sampling is allowed at the start and end of a render pass’s vertex and fragment stages, and at the start and end of compute and blit passes.
- [MTLCounterSamplingPoint.atTileDispatchBoundary](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint/attiledispatchboundary) — Counter sampling is allowed between tile dispatches in a render pass.
