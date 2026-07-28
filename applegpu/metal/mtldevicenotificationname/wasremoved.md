# wasRemoved

*Type Property · macOS 10.13*

<https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasremoved>

A notification that Metal sends to observers when the system removes a GPU device.

## Declaration

```swift
static let wasRemoved: MTLDeviceNotificationName
```

## Discussion

This notification tells your app that an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance and its methods are no longer valid to avoid runtime failures.

> **Important:**
>  If a person removes a GPU without warning, this notification may be posted without a prior [removalRequested](https://developer.apple.com/documentation/metal/mtldevicenotificationname/removalrequested) notification.

## See also

### Creating a notification name
- [wasAdded](https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasadded) — A notification that Metal sends to observers when the system adds a GPU device.
- [removalRequested](https://developer.apple.com/documentation/metal/mtldevicenotificationname/removalrequested) — A notification that Metal sends to observers when a person requests to remove a GPU device from the system.
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtldevicenotificationname/init(rawvalue:)) — Creates a Metal device notification name from a string.
