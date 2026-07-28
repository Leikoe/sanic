# MTLCopyAllDevices()

*Function · iOS 18.0, iPadOS 18.0, Mac Catalyst 13.0, macOS 10.11, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlcopyalldevices()>

Returns an array of all the Metal device instances in the system.

## Declaration

```swift
func MTLCopyAllDevices() -> [any MTLDevice]
```

## See also

### Locating GPUs
- [Finding multiple GPUs on an Intel-based Mac](https://developer.apple.com/documentation/metal/finding-multiple-gpus-on-an-intel-based-mac) — Locate, identify, and choose suitable GPUs for your app.
- [Getting the GPU that drives a view’s display](https://developer.apple.com/documentation/metal/getting-the-gpu-that-drives-a-views-display) — Keep up to date with the optimal device for your display.
- [MTLCopyAllDevicesWithObserver(handler:)](https://developer.apple.com/documentation/metal/mtlcopyalldeviceswithobserver(handler:)) — Returns an array of all the Metal GPU devices in the system and registers a notification handler that Metal calls when the device list changes.
- [MTLRemoveDeviceObserver(_:)](https://developer.apple.com/documentation/metal/mtlremovedeviceobserver(_:)) — Removes a registered observer of device notifications.
- [CGDirectDisplayCopyCurrentMetalDevice(_:)](https://developer.apple.com/documentation/CoreGraphics/CGDirectDisplayCopyCurrentMetalDevice(_:)) — Returns the GPU device instance that’s currently driving a display.
- [MTLDeviceNotificationHandler](https://developer.apple.com/documentation/metal/mtldevicenotificationhandler) — A Swift closure or an Objective-C block that Metal calls when the system adds or removes a GPU device.
- [MTLDeviceNotificationName](https://developer.apple.com/documentation/metal/mtldevicenotificationname) — A notification that represents a change to a GPU device in the system.
