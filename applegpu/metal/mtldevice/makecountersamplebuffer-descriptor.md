# makeCounterSampleBuffer(descriptor:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makecountersamplebuffer(descriptor:)>

Creates a counter sample buffer.

## Declaration

```swift
func makeCounterSampleBuffer(descriptor: MTLCounterSampleBufferDescriptor) throws -> any MTLCounterSampleBuffer
```

## Parameters

- **descriptor** — An [MTLCounterSampleBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor) instance.

## Return Value

A new [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## Discussion

The method may produce an error if the GPU driver has exhausted its underlying resources for counter sample buffers.

## See also

### Sampling a GPU device’s counters
- [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) — The counter sets supported by the device object.
- [supportsCounterSampling(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportscountersampling(_:)) — Returns a Boolean value that indicates whether you can read GPU counters at the specified command boundary.
- [MTLCounterSamplingPoint](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint) — Options for different times when you can sample GPU counters.
