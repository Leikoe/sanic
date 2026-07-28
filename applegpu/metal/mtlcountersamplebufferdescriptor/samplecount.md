# sampleCount

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/samplecount>

The number of instances of a counter set’s data that a counter sample buffer can store.

## Declaration

```swift
var sampleCount: Int { get set }
```

## Discussion

The counter sample buffer instances you create with the [MTLCounterSampleBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor) can store [sampleCount](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/samplecount) instances of a counter set.

## See also

### Configuring a descriptor for a counter sample buffer
- [counterSet](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/counterset) — A GPU device’s counter set instance that you want to sample.
- [label](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/label) — The name for the counter sample buffer you create with the descriptor.
- [storageMode](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/storagemode) — The memory storage mode for the counter sample buffers you create with the descriptor.
