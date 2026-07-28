# Adjusting for GPU memory bandwidth tradeoffs

*Article*

<https://developer.apple.com/documentation/metal/adjusting-for-gpu-memory-bandwidth-tradeoffs>

Choose a suitable GPU and memory storage mode for tasks based on that GPU’s memory bandwidth on a Mac.

## Overview

GPU memory *bandwidth* is a measure of the data transfer speed between a GPU and the system across a bus, such as PCI Express (PCIe) or Thunderbolt. It’s important to consider the bandwidth of each GPU in a system when developing your high-performance Metal apps. A GPU that’s powerful on its own may not be the optimal choice for certain tasks if it has a relatively low bandwidth connection to the system.

### Consider how a GPU connects to the system

A GPU’s bandwidth largely depends on the bus that connects it to a system:

- An *external* GPU connects to a system though an external Thunderbolt 3 bus.

- A *discrete* GPU is a built-in GPU that has video memory (separate memory that only the GPU can access) and connects to a system through an internal PCIe bus.

- An *integrated* GPU is a built-in GPU that uses system memory and shares the bus with the CPU.

![image](https://docs-assets.developer.apple.com/published/83c1259137eebf0aa4587150be955e6b/adjusting-for-gpu-memory-bandwidth-tradeoffs-1%402x.png)

A discrete GPU’s PCIe bus can have 8 or 16 memory lanes — or PCIe x4 or PCIe x16, respectively — depending on the GPU and Mac model. Transferring data between the system and an external GPU can take more time than with a built-in GPU because external GPUs typically have a lower bandwidth connection, such as Thunderbolt 3.

![image](https://docs-assets.developer.apple.com/published/94b6398aae2e8085a01a628a2dd31fa3/adjusting-for-gpu-memory-bandwidth-tradeoffs-2%402x.png)

Additionally, transferring data from one GPU to another can be even more expensive because the system can’t transfer data directly between GPUs. Instead, the process typically requires copying data to system memory before copying it again to the destination GPU.

### Select the appropriate storage mode for your resources

You can minimize the bandwidth costs — the number of data transfers across a bus — by selecting an appropriate storage mode for your app’s resources. For more information about selecting a storage mode for specific GPUs, see [Choosing a resource storage mode for Apple GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus) and [Choosing a resource storage mode for Intel and AMD GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-intel-and-amd-gpus). Metal uses a resource’s storage mode to determine which memory location to save it in. The storage mode options for a resource include the following:

- **[MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared)** — Shared resources reside in system memory and are slow to access for discrete and external GPUs.

- **[MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private)** — Private resources reside in video memory and are fast to access for discrete and external GPUs.

- **[MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed)** — Managed resources reside in both system and video memory (dual copies) and are fast to access for discrete and external GPUs.

Discrete and external GPUs have the highest data transfer costs when they access a shared resource because their access to system memory is relatively slow.

![image](https://docs-assets.developer.apple.com/published/28a42da176d4d3bb0248d39afaaa00cc/adjusting-for-gpu-memory-bandwidth-tradeoffs-6%402x.png)

Private resources have the lowest data transfer costs with discrete and external GPUs because their exclusive access to video memory is relatively fast.

![image](https://docs-assets.developer.apple.com/published/a48c90fa5d180d5d33e5fd95c8562847/adjusting-for-gpu-memory-bandwidth-tradeoffs-7%402x.png)

Managed resources can have modest data transfer costs for discrete and external GPUs. The CPU (and an integrated GPU) have quick access to the copy in system memory, and the other GPUs have quick access to the copy in their video memory.

![image](https://docs-assets.developer.apple.com/published/b01c231495f1bdcdf4d0af58141c56e6/adjusting-for-gpu-memory-bandwidth-tradeoffs-5%402x.png)

You can keep the copies in sync by efficiently running sparse blit operations (see [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos)).

### Render a drawable on the same GPU that drives the destination display

In Metal, a *drawable*, represented by [MTLDrawable](https://developer.apple.com/documentation/metal/mtldrawable), is a type that bridges Metal and [Core Animation](https://developer.apple.com/documentation/QuartzCore). Each drawable contains a texture that your apps can render with Metal and then present on a device’s display using Core Animation.

Presenting a drawable on a display can have significant bandwidth costs if the drawable belongs to a GPU that doesn’t drive the display. Only one GPU can drive a display, whether it’s built in or external, and the fastest path to present a drawable to a display is to render that drawable with the same GPU that drives the display. Otherwise, the system has to transfer the drawable from the GPU that renders it to the GPU that drives the display.

For example, take a Mac that has both a discrete GPU and an external GPU that’s driving an external display. If your app renders a drawable with the discrete GPU, the system has to transfer the drawable to the external GPU through the Thunderbolt 3 bus to present it on the external display. You app can avoid this transfer by rendering the drawable with the external GPU because it’s also driving the drawable’s destination display.

![image](https://docs-assets.developer.apple.com/published/3c0705eb46bb14d072b6534e22ae1ff0/adjusting-for-gpu-memory-bandwidth-tradeoffs-4%402x.png)

Similarly, Mac systems with multiple built-in GPUs may need to transfer a drawable that one GPU renders if a different GPU drives the destination display. For example, imagine a MacBook Pro with automatic graphics switching is currently driving the built-in display with the integrated GPU. If your app uses the discrete GPU to render a drawable, the system has to transfer the drawable’s contents to the integrated GPU over the internal PCIe bus. Your app can avoid this transfer by rendering the drawable with the integrated GPU when it’s driving the internal display.

![image](https://docs-assets.developer.apple.com/published/e2b8d38493c37785368e2efd35fa8578/adjusting-for-gpu-memory-bandwidth-tradeoffs-3%402x.png)

## See also

### Selecting GPUs
- [Assessing multi-GPU and multidisplay setups on an Intel-based Mac](https://developer.apple.com/documentation/metal/assessing-multi-gpu-and-multi-display-setups-on-an-intel-based-mac) — Learn the possible GPU and display configurations for a Mac and their limitations.
- [Selecting device objects for graphics rendering](https://developer.apple.com/documentation/metal/selecting-device-objects-for-graphics-rendering) — Switch dynamically between multiple GPUs to efficiently render to a display.
- [Selecting device objects for compute processing](https://developer.apple.com/documentation/metal/selecting-device-objects-for-compute-processing) — Switch dynamically between multiple GPUs to efficiently execute a compute-intensive simulation.
