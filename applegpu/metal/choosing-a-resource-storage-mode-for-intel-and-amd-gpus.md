# Choosing a resource storage mode for Intel and AMD GPUs

*Article*

<https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-intel-and-amd-gpus>

Select an appropriate storage mode for your textures and buffers on AMD and Intel GPUs.

## Overview

A Mac can have multiple GPUs, each with a unified or discrete memory model. In a *unified memory model*, the CPU and the GPU share system memory. However, CPU and GPU access to that memory depends on the storage mode you choose for your resources. In a *discrete memory model*, system memory is separate from video memory. System memory is accessible to both the CPU and the GPU, but video memory is accessible only to the GPU.

The Metal framework’s resource storage mode API works for both unified and discrete memory models, so you don’t need to write specific code for either.

### Understand the different Metal memory modes

In both memory models, a resource with an [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) mode resides in system memory accessible to both the CPU and the GPU. Shared resources are only available on systems with integrated graphics, such as Apple silicon and integrated GPUs on Intel-based Mac computers.

![image](https://docs-assets.developer.apple.com/published/90f2cc3c21c3f2151bddb3f0a744783d/choosing-a-resource-storage-mode-for-intel-and-amd-gpus-1%402x.png)

A resource with an [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) mode is accessible only to the GPU. In a unified memory model, this resource resides in system memory. In a discrete memory model, it resides in video memory. In both memory models, Metal optimizes GPU access to private resources.

![image](https://docs-assets.developer.apple.com/published/2bf54b6dd598f28f5df9ccab9b509206/choosing-a-resource-storage-mode-for-intel-and-amd-gpus-2%402x.png)

In a discrete memory model, Metal always attempts to store private resources in video memory. However, under certain memory constraints, Metal may evict a private resource into system memory. When you use a private resource that Metal previously evicted, Metal attempts to copy it back into video memory before you use it.

In a unified memory model, a resource with an [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed) mode resides in system memory accessible to both the CPU and the GPU.

In a discrete memory model, a managed resource exists as a synchronized pair of memory allocations. One copy of the resource resides in system memory accessible only to the CPU; the other resides in video memory accessible only to the GPU. However, you don’t manage the copies separately; Metal creates a single [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) instance to access both.

In both memory models, Metal optimizes CPU and GPU access to managed resources. However, you need to explicitly synchronize a managed resource after modifying its contents with the CPU or the GPU. For information about synchronizing a managed resource, see [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos).

![image](https://docs-assets.developer.apple.com/published/e0cc5e88d6f9223e3bec8a8c937a5558/choosing-a-resource-storage-mode-for-intel-and-amd-gpus-3%402x.png)

### Choose a storage mode for resources

Your storage mode should depend on the resource type and how your application uses storage accessed by Metal. Understanding the underlying memory architecture gives context and helps you investigate where you can optimize storage and synchronization. On Intel-based Mac computers, follow these guidelines:

- Prefer the default storage mode selected by Metal. Metal selects the optimal mode for the resource type and hardware.

- Use the [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) mode when creating a resource that’s only accessed by the GPU. This includes temporary targets for render passes.

- To optimize for workloads initialized from CPU and then only processed on GPU, copy from a CPU-populated resource in the default storage mode to a GPU-only [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) with [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) storage. See [Copying data to a private resource](https://developer.apple.com/documentation/metal/copying-data-to-a-private-resource) for details.

You’re responsible for signaling synchronization between the CPU and GPU with managed and shared storage. Regardless of your resource size, try and keep your synchronization points as light and infrequent as possible. Batch GPU work together to help reduce frequent synchronization. See [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos) for details.

To detect the GPU architecture and features at runtime, use the [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) method. See [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) for more information, and the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for full information on hardware support in Apple devices and computers.

## See also

### Resource management
- [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes) — Set a storage mode that defines the memory location and access permissions of a resource.
- [Choosing a resource storage mode for Apple GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus) — Select an appropriate storage mode for your textures and buffers on Apple GPUs.
- [Copying data to a private resource](https://developer.apple.com/documentation/metal/copying-data-to-a-private-resource) — Use a blit command encoder to copy buffer or texture data to a private resource.
- [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos) — Manually synchronize memory for a Metal resource in apps.
- [Transferring data between connected GPUs](https://developer.apple.com/documentation/metal/transferring-data-between-connected-gpus) — Use high-speed connections between GPUs to transfer data quickly.
- [Reducing the memory footprint of Metal apps](https://developer.apple.com/documentation/metal/reducing-the-memory-footprint-of-metal-apps) — Learn best practices for using memory efficiently in iOS and tvOS.
