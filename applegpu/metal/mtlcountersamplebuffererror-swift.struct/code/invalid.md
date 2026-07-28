# MTLCounterSampleBufferError.Code.invalid

*Case · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/code/invalid>

An error code that indicates when a counter-sample buffer descriptor has at least one invalid property.

## Declaration

```swift
case invalid
```

## Discussion

This error applies to the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) protocol’s [makeCounterSampleBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecountersamplebuffer(descriptor:)) method and its [MTLCounterSampleBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor) parameter.

## See also

### Error codes
- [MTLCounterSampleBufferError.Code.outOfMemory](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/code/outofmemory) — An error code that indicates the GPU device doesn’t have sufficient memory to create a counter sample buffer.
- [MTLCounterSampleBufferError.Code.internal](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/code/internal) — An error code that indicates the Metal framework has an internal problem.
- [MTLCounterSampleBufferError.Code.outOfMemory](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/code/outofmemory) — An error code that indicates the GPU device doesn’t have sufficient memory to create a counter sample buffer.
- [MTLCounterSampleBufferError.Code.internal](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct/code/internal) — An error code that indicates the Metal framework has an internal problem.
