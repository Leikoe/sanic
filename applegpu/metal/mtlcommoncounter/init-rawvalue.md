# init(rawValue:)

*Initializer · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommoncounter/init(rawvalue:)>

Creates a common counter name from a raw value.

## Declaration

```swift
init(rawValue: String)
```

## Parameters

- **rawValue** — The name of a common counter as a string.

## Discussion

Use of the [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) type’s static properties, such as [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounter/timestamp), [computeKernelInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/computekernelinvocations), or [totalCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/totalcycles) instead of creating a common counter instance yourself with this initializer.
