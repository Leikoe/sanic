# init(timestamp:)

*Initializer · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp/init(timestamp:)>

Creates a timestamp result from a value.

## Declaration

```swift
init(timestamp: UInt64)
```

## Parameters

- **timestamp** — A timestamp value from a counter sample buffer.

## Discussion

Metal creates [MTLCounterResultTimestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp) instances for you when you resolve the counter set’s data (see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format)). There’s no reason for you to manually create one in your app.

## See also

### Swift support
- [init()](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp/init()) — Creates a default timestamp result.
