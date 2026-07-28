# Choosing a resource storage mode for Apple GPUs

*Article*

<https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus>

Select an appropriate storage mode for your textures and buffers on Apple GPUs.

## Overview

Apple GPUs have a unified memory model in which the CPU and the GPU share system memory. However, CPU and GPU access to that memory depends on the storage mode you choose for your resources. The [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) mode defines system memory that both the CPU and the GPU can access. The [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) mode defines system memory that only the GPU can access.

The [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless) mode defines tile memory within the GPU that only the GPU can access. Tile memory has higher bandwidth, lower latency, and consumes less power than system memory.

![image](https://docs-assets.developer.apple.com/published/95106a9e6960adc249245b78fad36f76/choosing-a-resource-storage-mode-for-apple-gpus-1%402x.png)

### Choose a resource storage mode for buffers or textures

The storage mode you choose depends on how you plan to use Metal resources:

- **Populate and update on the CPU** — Data shared by the CPU and GPU. Use [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared). The CPU and GPU share data. This is the default for buffer and texture storage.

- **Access exclusively on the GPU** — Data owned by the GPU. Use [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private). Choose the mode if you populate your resource with the GPU through a compute, render, or blit pass. This case is common for render targets, intermediary resources, or texture streaming. For guidance on how to copy data to a private resource, see [Copying data to a private resource](https://developer.apple.com/documentation/metal/copying-data-to-a-private-resource).

- **Populate on CPU and access frequently on GPU** — Shared integrated memory for the CPU and GPU. Use [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared).

- **Temporary texture contents for GPU passes** — Memory held by the GPU for textures within or between passes. Use [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless). Memoryless mode only works for textures, and stores temporary resources in tiled memory for high performance. An example is a depth or stencil texture thatʼs used only within a single pass and isnʼt needed in an earlier or later rendering stage.

For information on setting storage modes in your app, see [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes).

### Create a memoryless render target

To create a memoryless render target, set the [storageMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/storagemode) property of an [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) to [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless) and use this descriptor to create a new [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture). Then set this new texture as the [texture](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/texture) property of an [MTLRenderPassAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor).

```swift
let memorylessDescriptor = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .r16Float,
                                                                    width: 256,
                                                                    height: 256,
                                                                    mipmapped: true)
memorylessDescriptor.storageMode = .memoryless
let memorylessTexture = device.makeTexture(descriptor: memorylessDescriptor)

let renderPassDescriptor = MTLRenderPassDescriptor()
renderPassDescriptor.depthAttachment.texture = memorylessTexture
```

```objective-c
MTLTextureDescriptor *memorylessDescriptor = [MTLTextureDescriptor texture2DDescriptorWithPixelFormat:MTLPixelFormatR16Float
                                                                                                width:256
                                                                                               height:256
                                                                                            mipmapped:YES];
memorylessDescriptor.storageMode = MTLStorageModeMemoryless;
id <MTLTexture> memorylessTexture = [_device newTextureWithDescriptor:memorylessDescriptor];
    
MTLRenderPassDescriptor *renderPassDescriptor = [MTLRenderPassDescriptor renderPassDescriptor];
renderPassDescriptor.depthAttachment.texture = memorylessTexture;
```

See [Rendering a scene with deferred lighting in Objective-C](https://developer.apple.com/documentation/metal/rendering-a-scene-with-deferred-lighting-in-objective-c) for an example of an app that uses a memoryless render target.

> **Note:**
>  You can create only textures, not buffers, using [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless) mode. You can’t use buffers as memoryless render targets.

## See also

### Resource management
- [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes) — Set a storage mode that defines the memory location and access permissions of a resource.
- [Choosing a resource storage mode for Intel and AMD GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-intel-and-amd-gpus) — Select an appropriate storage mode for your textures and buffers on AMD and Intel GPUs.
- [Copying data to a private resource](https://developer.apple.com/documentation/metal/copying-data-to-a-private-resource) — Use a blit command encoder to copy buffer or texture data to a private resource.
- [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos) — Manually synchronize memory for a Metal resource in apps.
- [Transferring data between connected GPUs](https://developer.apple.com/documentation/metal/transferring-data-between-connected-gpus) — Use high-speed connections between GPUs to transfer data quickly.
- [Reducing the memory footprint of Metal apps](https://developer.apple.com/documentation/metal/reducing-the-memory-footprint-of-metal-apps) — Learn best practices for using memory efficiently in iOS and tvOS.
