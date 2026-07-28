# label

*Instance Property · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor/label>

An optional name that can help you identify a residency set you create with the descriptor.

## Declaration

```swift
var label: String? { get set }
```

## Discussion

Metal applies the value of this property to the [label](https://developer.apple.com/documentation/metal/mtlresidencyset/label) property of an [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) that you create by passing the descriptor to [makeResidencySet(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeresidencyset(descriptor:)).

## See also

### Configuring the residency set
- [initialCapacity](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor/initialcapacity) — The number of allocations a new residency set can store without reallocating memory.
