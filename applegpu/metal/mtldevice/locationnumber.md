# locationNumber

*Instance Property · macOS 10.15*

<https://developer.apple.com/documentation/metal/mtldevice/locationnumber>

A specific GPU position based on its general location.

## Declaration

```swift
var locationNumber: Int { get }
```

## Discussion

The meaning of the location number depends on a device’s [location](https://developer.apple.com/documentation/metal/mtldevice/location) property:

- For [MTLDeviceLocation.builtIn](https://developer.apple.com/documentation/metal/mtldevicelocation/builtin), the location number is `0` for low-power GPUs (see [isLowPower](https://developer.apple.com/documentation/metal/mtldevice/islowpower)) and `1` for other GPUs.

- For [MTLDeviceLocation.slot](https://developer.apple.com/documentation/metal/mtldevicelocation/slot), the location number represents the slot.

- For [MTLDeviceLocation.external](https://developer.apple.com/documentation/metal/mtldevicelocation/external), the location number represents the Thunderbolt port.

> **Note:**
>  It’s possible for multiple devices to share the same location and number. For example, a card in a slot may have multiple GPUs, or a person may connect multiple eGPUs to the same Thunderbolt port.

## See also

### Identifying a GPU device
- [name](https://developer.apple.com/documentation/metal/mtldevice/name) — The full name of the GPU device.
- [architecture](https://developer.apple.com/documentation/metal/mtldevice/architecture) — The architectural details of the GPU device.
- [MTLArchitecture](https://developer.apple.com/documentation/metal/mtlarchitecture) — A class that contains the architectural details of a GPU device.
- [registryID](https://developer.apple.com/documentation/metal/mtldevice/registryid) — The GPU device’s registry identifier.
- [location](https://developer.apple.com/documentation/metal/mtldevice/location) — The physical location of the GPU relative to the system.
- [MTLDeviceLocation](https://developer.apple.com/documentation/metal/mtldevicelocation) — Indicates the location of the GPU relative to the system it’s connect to.
- [isLowPower](https://developer.apple.com/documentation/metal/mtldevice/islowpower) — A Boolean value that indicates whether the GPU lowers its performance to conserve energy.
- [isRemovable](https://developer.apple.com/documentation/metal/mtldevice/isremovable) — A Boolean value that indicates whether the GPU is removable.
- [isHeadless](https://developer.apple.com/documentation/metal/mtldevice/isheadless) — A Boolean value that indicates whether a GPU device doesn’t have a connection to a display.
- [peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid) — The peer group ID the GPU belongs to, if applicable.
- [peerCount](https://developer.apple.com/documentation/metal/mtldevice/peercount) — The total number of GPUs in the peer group, if applicable.
- [peerIndex](https://developer.apple.com/documentation/metal/mtldevice/peerindex) — The unique identifier for a GPU in a peer group.
