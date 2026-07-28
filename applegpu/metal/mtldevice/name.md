# name

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/name>

The full name of the GPU device.

## Declaration

```swift
var name: String { get }
```

## See also

### Identifying a GPU device
- [architecture](https://developer.apple.com/documentation/metal/mtldevice/architecture) — The architectural details of the GPU device.
- [MTLArchitecture](https://developer.apple.com/documentation/metal/mtlarchitecture) — A class that contains the architectural details of a GPU device.
- [registryID](https://developer.apple.com/documentation/metal/mtldevice/registryid) — The GPU device’s registry identifier.
- [location](https://developer.apple.com/documentation/metal/mtldevice/location) — The physical location of the GPU relative to the system.
- [MTLDeviceLocation](https://developer.apple.com/documentation/metal/mtldevicelocation) — Indicates the location of the GPU relative to the system it’s connect to.
- [locationNumber](https://developer.apple.com/documentation/metal/mtldevice/locationnumber) — A specific GPU position based on its general location.
- [isLowPower](https://developer.apple.com/documentation/metal/mtldevice/islowpower) — A Boolean value that indicates whether the GPU lowers its performance to conserve energy.
- [isRemovable](https://developer.apple.com/documentation/metal/mtldevice/isremovable) — A Boolean value that indicates whether the GPU is removable.
- [isHeadless](https://developer.apple.com/documentation/metal/mtldevice/isheadless) — A Boolean value that indicates whether a GPU device doesn’t have a connection to a display.
- [peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid) — The peer group ID the GPU belongs to, if applicable.
- [peerCount](https://developer.apple.com/documentation/metal/mtldevice/peercount) — The total number of GPUs in the peer group, if applicable.
- [peerIndex](https://developer.apple.com/documentation/metal/mtldevice/peerindex) — The unique identifier for a GPU in a peer group.
