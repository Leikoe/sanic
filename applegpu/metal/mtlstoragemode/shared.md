# MTLStorageMode.shared

*Case · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstoragemode/shared>

The CPU and GPU share access to the resource, allocated in system memory.

## Declaration

```swift
case shared
```

## Discussion

This is the default storage mode for [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instances on integrated GPUs and both [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) and [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instances on Apple silicon GPUs. On non-Apple family GPUs, the shared storage mode isn’t available for [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instances.

When either the CPU or GPU changes the contents of the resource, you’re responsible for synchronizing access to the texture from the other participant. Ensure that all changes you schedule on either the CPU or GPU for a resource that uses shared memory complete before accessing that resource on the other processor.

For more guidance on how to choose storage modes, see [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes).

## See also

### Storage mode options
- [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed) — The CPU and GPU may maintain separate copies of the resource, and any changes need to be explicitly synchronized.
- [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) — The resource is only available to the GPU.
- [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless) — The resource’s contents are only available to the GPU, and only exist temporarily during a render pass.
