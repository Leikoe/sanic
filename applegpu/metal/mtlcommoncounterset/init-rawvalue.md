# init(rawValue:)

*Initializer · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommoncounterset/init(rawvalue:)>

Creates a common counter set name from a raw value.

## Declaration

```swift
init(rawValue: String)
```

## Parameters

- **rawValue** — The name of a counter set as a string.

## Discussion

Use one of the [MTLCommonCounterSet](https://developer.apple.com/documentation/metal/mtlcommoncounterset) type’s static properties, such as [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp), [stageUtilization](https://developer.apple.com/documentation/metal/mtlcommoncounterset/stageutilization), and [statistic](https://developer.apple.com/documentation/metal/mtlcommoncounterset/statistic) instead of creating a common counter set instance yourself with this initializer.
