# storageModePrivate

*Type Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeprivate>

The resource is only available to the GPU.

## Declaration

```swift
static var storageModePrivate: MTLResourceOptions { get }
```

## Discussion

Metal may apply additional optimizations to private resources that aren’t allowed on shared or managed resources.

For more guidance on how to choose storage modes, see [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes).

## See also

### Specifying storage modes
- [storageModeShared](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeshared) — The CPU and GPU share access to the resource, allocated in system memory.
- [storageModeManaged](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodemanaged) — The CPU and GPU may maintain separate copies of the resource, and any changes need to be explicitly synchronized.
- [storageModeMemoryless](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodememoryless) — The resource’s contents are only available to the GPU, and only exist temporarily during a render pass.
