# MTLCreateSystemDefaultDevice()

*Function · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcreatesystemdefaultdevice()>

Returns the device instance Metal selects as the default.

## Declaration

```swift
func MTLCreateSystemDefaultDevice() -> (any MTLDevice)?
```

## Return Value

A device object.

## Discussion

In macOS, in order for the system to provide a default Metal device object, you need to link to the [Core Graphics](https://developer.apple.com/documentation/CoreGraphics) framework. You usually need to do this explicitly if you’re writing apps that don’t use graphics by default, such as command line tools.

## See also

### Locating and inspecting a GPU device
- [Getting the default GPU](https://developer.apple.com/documentation/metal/getting-the-default-gpu) — Select the system’s default GPU device on which to run your Metal code.
- [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) — Use the device object’s properties to determine how you perform tasks in Metal.
- [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) — The main Metal interface to a GPU that apps use to draw graphics and run computations in parallel.
- [Multi-GPU systems](https://developer.apple.com/documentation/metal/multi-gpu-systems) — Locate and work with internal and external GPUs and their displays, video memory, and performance tradeoffs.
