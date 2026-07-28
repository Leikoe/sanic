# Setting resource storage modes

*Article*

<https://developer.apple.com/documentation/metal/setting-resource-storage-modes>

Set a storage mode that defines the memory location and access permissions of a resource.

## Overview

Storage modes are only set when creating an instance, and the system default allows for access to memory from both the CPU and GPU. Metal selects the default mode for resources depending on hardware.

- For Apple silicon GPUs the default is [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared).

- For Intel-based Mac computers, the default is [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed) for all [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instances and [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instances attached to discrete GPUs. [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instances using the integrated GPU have [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) as their default.

> **Important:**
>  Use the system default if your data is available to both the CPU and GPU. When you manually select shared or managed mode, your app may not run on some hardware.

You perform the same synchronization tasks to ensure GPU and CPU memory coherency in both default modes. To check for GPU architecture and capabilities, use the [supportsFamily(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsfamily(_:)) method instead of the [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) property. See [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) for more information.

Use [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless), only available on Apple silicon, when you manage your own storage, or want to run a GPU task that requires temporary resources. For tasks that share memory on the GPU, use [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) storage. This article includes examples of how to set the storage mode for a buffer or texture.

For more guidance on which mode to choose, see [Choosing a resource storage mode for Apple GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus) and [Choosing a resource storage mode for Intel and AMD GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-intel-and-amd-gpus).

### Set a storage mode for a buffer

Create a new [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) with the [makeBuffer(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:)) method and set its storage mode in the method’s `options` parameter.

```swift
let bufferOptions = MTLResourceOptions.storageModePrivate
let buffer = device.makeBuffer(length: 256,
                               options: bufferOptions)
```

```objective-c
MTLResourceOptions bufferOptions = MTLResourceStorageModePrivate;
id <MTLBuffer> buffer = [_device newBufferWithLength:256
                                             options:bufferOptions];
```

> **Note:**
>  The storage mode options in [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) are equivalent to the storage mode values in [MTLStorageMode](https://developer.apple.com/documentation/metal/mtlstoragemode). When you create a new buffer, you can combine multiple resource options but you can set only one storage mode.

### Set a storage mode for a texture

Create a new [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) and set its storage mode in the descriptor’s [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) property. Then create a new [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) with the [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)) method.

```swift
let textureDescriptor = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .bgra8Unorm,
                                                                 width: 256,
                                                                 height: 256,
                                                                 mipmapped: true)
textureDescriptor.storageMode = .private
let texture = device.makeTexture(descriptor: textureDescriptor)
```

```objective-c
MTLTextureDescriptor *textureDescriptor = [MTLTextureDescriptor texture2DDescriptorWithPixelFormat:MTLPixelFormatBGRA8Unorm
                                                                                             width:256
                                                                                            height:256
                                                                                         mipmapped:YES];
textureDescriptor.storageMode = MTLStorageModePrivate;
id <MTLTexture> texture = [_device newTextureWithDescriptor:textureDescriptor];
```

## See also

### Resource management
- [Choosing a resource storage mode for Apple GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus) — Select an appropriate storage mode for your textures and buffers on Apple GPUs.
- [Choosing a resource storage mode for Intel and AMD GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-intel-and-amd-gpus) — Select an appropriate storage mode for your textures and buffers on AMD and Intel GPUs.
- [Copying data to a private resource](https://developer.apple.com/documentation/metal/copying-data-to-a-private-resource) — Use a blit command encoder to copy buffer or texture data to a private resource.
- [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos) — Manually synchronize memory for a Metal resource in apps.
- [Transferring data between connected GPUs](https://developer.apple.com/documentation/metal/transferring-data-between-connected-gpus) — Use high-speed connections between GPUs to transfer data quickly.
- [Reducing the memory footprint of Metal apps](https://developer.apple.com/documentation/metal/reducing-the-memory-footprint-of-metal-apps) — Learn best practices for using memory efficiently in iOS and tvOS.
