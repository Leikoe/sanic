# storageModeManaged

*Type Property · Mac Catalyst 13.0, macOS 10.11*

<https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodemanaged>

The CPU and GPU may maintain separate copies of the resource, and any changes need to be explicitly synchronized.

## Declaration

```swift
static var storageModeManaged: MTLResourceOptions { get }
```

## Discussion

On Intel-based Mac computers, this is the default storage mode for [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) objects. In iOS and tvOS, the managed storage mode isn’t available. With managed storage, you synchronize changes between the CPU and GPU manually. For instructions and examples of resource synchronization, see [Synchronizing a managed resource in macOS](https://developer.apple.com/documentation/metal/synchronizing-a-managed-resource-in-macos).

For more guidance on how to choose storage modes, see [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes).

## See also

### Specifying storage modes
- [storageModeShared](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeshared) — The CPU and GPU share access to the resource, allocated in system memory.
- [storageModePrivate](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeprivate) — The resource is only available to the GPU.
- [storageModeMemoryless](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodememoryless) — The resource’s contents are only available to the GPU, and only exist temporarily during a render pass.
