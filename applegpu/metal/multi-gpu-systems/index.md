# Multi-GPU systems

*API Collection*

<https://developer.apple.com/documentation/metal/multi-gpu-systems>

Locate and work with internal and external GPUs and their displays, video memory, and performance tradeoffs.

## Overview

Your app can submit work to any or all of the GPUs of a system that supports multiple GPUs. For example, every Mac notebook, such as a MacBook Pro, has an internal GPU, but some have two.

![image](https://docs-assets.developer.apple.com/published/b022119842cf3a4a9bc64718e4ac3a20/assessing-multi-gpu-and-multi-display-setups-on-an-intel-based-mac-3%402x.png)

A Mac may have a Thunderbolt connection to an external GPU and its displays.

![image](https://docs-assets.developer.apple.com/published/b22a3f95645c4220377c91079b7fb6c5/assessing-multi-gpu-and-multi-display-setups-on-an-intel-based-mac-6%402x.png)

Some systems may have even more complicated arrangements of internal and multiple external GPUs and displays.

![image](https://docs-assets.developer.apple.com/published/b505af846a78d0167e779ce702fb7d61/assessing-multi-gpu-and-multi-display-setups-on-an-intel-based-mac-7%402x.png)

For more information about Mac configurations with GPUs and displays, see [Assessing multi-GPU and multidisplay setups on an Intel-based Mac](https://developer.apple.com/documentation/metal/assessing-multi-gpu-and-multi-display-setups-on-an-intel-based-mac).

Start by locating all GPUs in a system and identifying their types (see [Finding multiple GPUs on an Intel-based Mac](https://developer.apple.com/documentation/metal/finding-multiple-gpus-on-an-intel-based-mac)). Alternatively, you can locate a specific GPU that’s driving a display (see [Getting the GPU that drives a view’s display](https://developer.apple.com/documentation/metal/getting-the-gpu-that-drives-a-views-display)).

When selecting a GPU, consider its memory bandwidth and the storage mode options for its memory resources (see [Adjusting for GPU memory bandwidth tradeoffs](https://developer.apple.com/documentation/metal/adjusting-for-gpu-memory-bandwidth-tradeoffs)).

For examples of how to use external GPUs in your graphics rendering or compute processing workflows, see the following:

- [Selecting device objects for graphics rendering](https://developer.apple.com/documentation/metal/selecting-device-objects-for-graphics-rendering)

- [Selecting device objects for compute processing](https://developer.apple.com/documentation/metal/selecting-device-objects-for-compute-processing)

For more information about external GPU configurations, see [Use an external graphics processor with your Mac](https://support.apple.com/kb/HT208544).

> **Note:**
>  The system may gain or lose an external GPU at any time (see [Handling external GPU additions and removals](https://developer.apple.com/documentation/metal/handling-external-gpu-additions-and-removals)).

## Topics

### Locating GPUs
- [Finding multiple GPUs on an Intel-based Mac](https://developer.apple.com/documentation/metal/finding-multiple-gpus-on-an-intel-based-mac) — Locate, identify, and choose suitable GPUs for your app.
- [Getting the GPU that drives a view’s display](https://developer.apple.com/documentation/metal/getting-the-gpu-that-drives-a-views-display) — Keep up to date with the optimal device for your display.
- [MTLCopyAllDevices()](https://developer.apple.com/documentation/metal/mtlcopyalldevices()) — Returns an array of all the Metal device instances in the system.
- [MTLCopyAllDevicesWithObserver(handler:)](https://developer.apple.com/documentation/metal/mtlcopyalldeviceswithobserver(handler:)) — Returns an array of all the Metal GPU devices in the system and registers a notification handler that Metal calls when the device list changes.
- [MTLRemoveDeviceObserver(_:)](https://developer.apple.com/documentation/metal/mtlremovedeviceobserver(_:)) — Removes a registered observer of device notifications.
- [CGDirectDisplayCopyCurrentMetalDevice(_:)](https://developer.apple.com/documentation/CoreGraphics/CGDirectDisplayCopyCurrentMetalDevice(_:)) — Returns the GPU device instance that’s currently driving a display.
- [MTLDeviceNotificationHandler](https://developer.apple.com/documentation/metal/mtldevicenotificationhandler) — A Swift closure or an Objective-C block that Metal calls when the system adds or removes a GPU device.
- [MTLDeviceNotificationName](https://developer.apple.com/documentation/metal/mtldevicenotificationname) — A notification that represents a change to a GPU device in the system.

### Selecting GPUs
- [Adjusting for GPU memory bandwidth tradeoffs](https://developer.apple.com/documentation/metal/adjusting-for-gpu-memory-bandwidth-tradeoffs) — Choose a suitable GPU and memory storage mode for tasks based on that GPU’s memory bandwidth on a Mac.
- [Assessing multi-GPU and multidisplay setups on an Intel-based Mac](https://developer.apple.com/documentation/metal/assessing-multi-gpu-and-multi-display-setups-on-an-intel-based-mac) — Learn the possible GPU and display configurations for a Mac and their limitations.
- [Selecting device objects for graphics rendering](https://developer.apple.com/documentation/metal/selecting-device-objects-for-graphics-rendering) — Switch dynamically between multiple GPUs to efficiently render to a display.
- [Selecting device objects for compute processing](https://developer.apple.com/documentation/metal/selecting-device-objects-for-compute-processing) — Switch dynamically between multiple GPUs to efficiently execute a compute-intensive simulation.

### Working with external GPUs
- [Handling external GPU additions and removals](https://developer.apple.com/documentation/metal/handling-external-gpu-additions-and-removals) — Register and respond to external GPU notifications that a person initiates.
- [Transferring data between connected GPUs](https://developer.apple.com/documentation/metal/transferring-data-between-connected-gpus) — Use high-speed connections between GPUs to transfer data quickly.

## See also

### Locating and inspecting a GPU device
- [Getting the default GPU](https://developer.apple.com/documentation/metal/getting-the-default-gpu) — Select the system’s default GPU device on which to run your Metal code.
- [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) — Use the device object’s properties to determine how you perform tasks in Metal.
- [MTLCreateSystemDefaultDevice()](https://developer.apple.com/documentation/metal/mtlcreatesystemdefaultdevice()) — Returns the device instance Metal selects as the default.
- [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) — The main Metal interface to a GPU that apps use to draw graphics and run computations in parallel.
