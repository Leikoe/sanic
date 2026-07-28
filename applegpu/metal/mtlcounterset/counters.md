# counters

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterset/counters>

An array of the counter instances a GPU device supports.

## Declaration

```swift
var counters: [any MTLCounter] { get }
```

## Discussion

Check whether a GPU device supports a specific counter by comparing its common name (see [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter)) with each element in the property’s array.

> **Important:**
>  Some GPUs may only support some of the counters within a counter set.

For more information, see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports).
