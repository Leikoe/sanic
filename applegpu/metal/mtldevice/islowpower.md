# isLowPower

*Instance Property · Mac Catalyst 13.0, macOS 10.11*

<https://developer.apple.com/documentation/metal/mtldevice/islowpower>

A Boolean value that indicates whether the GPU lowers its performance to conserve energy.

## Declaration

```swift
var isLowPower: Bool { get }
```

## Discussion

Some systems contain multiple GPUs that run with different performance and energy characteristics. At runtime, choose a GPU that best matches your performance needs while considering the current state of the system. For example, your app may choose a lower-power GPU if it doesn’t need the best possible performance on a MacBook Pro that’s running on battery power. For more information on discovering and selecting GPUs at runtime, see [Multi-GPU systems](https://developer.apple.com/documentation/metal/multi-gpu-systems).

> **Note:**
>  Systems with Apple silicon only have one GPU, which removes the need to choose a GPU.

The property is typically [true](https://developer.apple.com/documentation/Swift/true) for integrated GPUs and [false](https://developer.apple.com/documentation/Swift/false) for discrete GPUs. However, an Apple silicon GPU on a Mac sets the property to [false](https://developer.apple.com/documentation/Swift/false) because it doesn’t need to lower its performance to conserve energy.

## See also

### Identifying a GPU device
- [name](https://developer.apple.com/documentation/metal/mtldevice/name) — The full name of the GPU device.
- [architecture](https://developer.apple.com/documentation/metal/mtldevice/architecture) — The architectural details of the GPU device.
- [MTLArchitecture](https://developer.apple.com/documentation/metal/mtlarchitecture) — A class that contains the architectural details of a GPU device.
- [registryID](https://developer.apple.com/documentation/metal/mtldevice/registryid) — The GPU device’s registry identifier.
- [location](https://developer.apple.com/documentation/metal/mtldevice/location) — The physical location of the GPU relative to the system.
- [MTLDeviceLocation](https://developer.apple.com/documentation/metal/mtldevicelocation) — Indicates the location of the GPU relative to the system it’s connect to.
- [locationNumber](https://developer.apple.com/documentation/metal/mtldevice/locationnumber) — A specific GPU position based on its general location.
- [isRemovable](https://developer.apple.com/documentation/metal/mtldevice/isremovable) — A Boolean value that indicates whether the GPU is removable.
- [isHeadless](https://developer.apple.com/documentation/metal/mtldevice/isheadless) — A Boolean value that indicates whether a GPU device doesn’t have a connection to a display.
- [peerGroupID](https://developer.apple.com/documentation/metal/mtldevice/peergroupid) — The peer group ID the GPU belongs to, if applicable.
- [peerCount](https://developer.apple.com/documentation/metal/mtldevice/peercount) — The total number of GPUs in the peer group, if applicable.
- [peerIndex](https://developer.apple.com/documentation/metal/mtldevice/peerindex) — The unique identifier for a GPU in a peer group.
