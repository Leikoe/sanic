# isRemovable

*Instance Property · Mac Catalyst 13.0, macOS 10.13*

<https://developer.apple.com/documentation/metal/mtldevice/isremovable>

A Boolean value that indicates whether the GPU is removable.

## Declaration

```swift
var isRemovable: Bool { get }
```

## Discussion

You can respond to GPU removal notifications by registering with the [MTLCopyAllDevicesWithObserver(handler:)](https://developer.apple.com/documentation/metal/mtlcopyalldeviceswithobserver(handler:)) function in Swift, or the [MTLCopyAllDevicesWithObserver](https://developer.apple.com/documentation/metal/mtlcopyalldeviceswithobserver) function in Objective-C, and responding to the [removalRequested](https://developer.apple.com/documentation/metal/mtldevicenotificationname/removalrequested) and [wasRemoved](https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasremoved) device notification names.

> **Important:**
>  If a person removes a GPU without warning, [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) APIs may fail even before your app receives a [wasRemoved](https://developer.apple.com/documentation/metal/mtldevicenotificationname/wasremoved) notification.

## See also

### Identifying a GPU device
- [name](https://developer.apple.com/documentation/metal/mtldevice/name) — The full name of the GPU device.
- [architecture](https://developer.apple.com/documentation/metal/mtldevice/architecture) — The architectural details of the GPU device.
- [MTLArchitecture](https://developer.apple.com/documentation/metal/mtlarchitecture) — A class that contains the architectural details of a GPU device.
- [registryID](https://developer.apple.com/documentation/metal/mtldevice/registryid) — The GPU device’s registry identifier.
- [location](https://developer.apple.com/documentation/metal/mtldevice/location) — The physical location of the GPU relative to the system.
- [MTLDeviceLocation](https://developer.apple.com/documentation/metal/mtldevicelocation) — Indicates the location of the GPU relative to the system it’s connect to.
- [locationNumber](https://developer.apple.com/documentation/metal/mtldevice/locationnumber) — A specific GPU position based on its general location.
- [isLowPower](https://developer.apple.com/documentation/metal/mtldevice/islowpower) — A Boolean value that indicates whether the GPU lowers its performance to conserve energy.
- [isHeadless](https://developer.apple.com/documentation/metal/mtldevice/isheadless) — A Boolean value that indicates whether a GPU device doesn’t have a connection to a display.
- [peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid) — The peer group ID the GPU belongs to, if applicable.
- [peerCount](https://developer.apple.com/documentation/metal/mtldevice/peercount) — The total number of GPUs in the peer group, if applicable.
- [peerIndex](https://developer.apple.com/documentation/metal/mtldevice/peerindex) — The unique identifier for a GPU in a peer group.
