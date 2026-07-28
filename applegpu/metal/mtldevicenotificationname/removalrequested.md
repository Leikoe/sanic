# removalRequested

*Type Property · macOS 10.13*

<https://developer.apple.com/documentation/metal/mtldevicenotificationname/removalrequested>

A notification that Metal sends to observers when a person requests to remove a GPU device from the system.

## Declaration

```swift
static let removalRequested: MTLDeviceNotificationName
```

## Discussion

This notification tells your app to stop using an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance by releasing any objects and resources your app created with it.

> **Note:**
>  Metal removes the device instance from the array it returns with its methods — such as [MTLCopyAllDevices()](https://developer.apple.com/documentation/metal/mtlcopyalldevices()) — before sending this notification.

## See also

### Creating a notification name
- [wasAdded](https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasadded) — A notification that Metal sends to observers when the system adds a GPU device.
- [wasRemoved](https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasremoved) — A notification that Metal sends to observers when the system removes a GPU device.
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtldevicenotificationname/init(rawvalue:)) — Creates a Metal device notification name from a string.
