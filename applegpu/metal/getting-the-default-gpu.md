# Getting the default GPU

*Article*

<https://developer.apple.com/documentation/metal/getting-the-default-gpu>

Select the system’s default GPU device on which to run your Metal code.

## Overview

To use the Metal framework, start by getting a GPU device. All of the instances your app needs to interact with Metal come from an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) that you acquire at runtime. Some devices, such as those with iOS and tvOS have a single GPU that you can access by calling [MTLCreateSystemDefaultDevice()](https://developer.apple.com/documentation/metal/mtlcreatesystemdefaultdevice()).

```swift
if(!(device = MTLCreateSystemDefaultDevice()))
{
    NSLog(@"Failed to get the system's default Metal device.");
}
```

On macOS devices that have multiple GPUs, such as a MacBook Pro, the system default is the discrete GPU.

## See also

### Locating and inspecting a GPU device
- [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) — Use the device object’s properties to determine how you perform tasks in Metal.
- [MTLCreateSystemDefaultDevice()](https://developer.apple.com/documentation/metal/mtlcreatesystemdefaultdevice()) — Returns the device instance Metal selects as the default.
- [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) — The main Metal interface to a GPU that apps use to draw graphics and run computations in parallel.
- [Multi-GPU systems](https://developer.apple.com/documentation/metal/multi-gpu-systems) — Locate and work with internal and external GPUs and their displays, video memory, and performance tradeoffs.
