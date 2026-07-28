# counterSet

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/counterset>

A GPU device’s counter set instance that you want to sample.

## Declaration

```swift
var counterSet: (any MTLCounterSet)? { get set }
```

## Discussion

Assign this property to one of the counter sets in an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) property.

## See also

### Configuring a descriptor for a counter sample buffer
- [label](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/label) — The name for the counter sample buffer you create with the descriptor.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/samplecount) — The number of instances of a counter set’s data that a counter sample buffer can store.
- [storageMode](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor/storagemode) — The memory storage mode for the counter sample buffers you create with the descriptor.
