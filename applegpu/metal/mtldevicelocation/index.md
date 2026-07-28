# MTLDeviceLocation

*Enumeration · macOS 10.15*

<https://developer.apple.com/documentation/metal/mtldevicelocation>

Indicates the location of the GPU relative to the system it’s connect to.

## Declaration

```swift
enum MTLDeviceLocation
```

## Overview

Check the location of a GPU by checking the [location](https://developer.apple.com/documentation/metal/mtldevice/location) property of its [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

## Topics

### Determining the GPU’s location
- [MTLDeviceLocation.builtIn](https://developer.apple.com/documentation/metal/mtldevicelocation/builtin) — A location that indicates the GPU is permanently connected to the system internally.
- [MTLDeviceLocation.slot](https://developer.apple.com/documentation/metal/mtldevicelocation/slot) — A GPU location that indicates a person connected the GPU to a system’s internal slot.
- [MTLDeviceLocation.external](https://developer.apple.com/documentation/metal/mtldevicelocation/external) — A GPU location that indicates a person connected the GPU to the system with an external interface, such as Thunderbolt.
- [MTLDeviceLocation.unspecified](https://developer.apple.com/documentation/metal/mtldevicelocation/unspecified) — A value that indicates the system can’t determine how the GPU connects to it.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtldevicelocation/init(rawvalue:))

## See also

### Identifying a GPU device
- [name](https://developer.apple.com/documentation/metal/mtldevice/name) — The full name of the GPU device.
- [architecture](https://developer.apple.com/documentation/metal/mtldevice/architecture) — The architectural details of the GPU device.
- [MTLArchitecture](https://developer.apple.com/documentation/metal/mtlarchitecture) — A class that contains the architectural details of a GPU device.
- [registryID](https://developer.apple.com/documentation/metal/mtldevice/registryid) — The GPU device’s registry identifier.
- [location](https://developer.apple.com/documentation/metal/mtldevice/location) — The physical location of the GPU relative to the system.
- [locationNumber](https://developer.apple.com/documentation/metal/mtldevice/locationnumber) — A specific GPU position based on its general location.
- [isLowPower](https://developer.apple.com/documentation/metal/mtldevice/islowpower) — A Boolean value that indicates whether the GPU lowers its performance to conserve energy.
- [isRemovable](https://developer.apple.com/documentation/metal/mtldevice/isremovable) — A Boolean value that indicates whether the GPU is removable.
- [isHeadless](https://developer.apple.com/documentation/metal/mtldevice/isheadless) — A Boolean value that indicates whether a GPU device doesn’t have a connection to a display.
- [peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid) — The peer group ID the GPU belongs to, if applicable.
- [peerCount](https://developer.apple.com/documentation/metal/mtldevice/peercount) — The total number of GPUs in the peer group, if applicable.
- [peerIndex](https://developer.apple.com/documentation/metal/mtldevice/peerindex) — The unique identifier for a GPU in a peer group.
