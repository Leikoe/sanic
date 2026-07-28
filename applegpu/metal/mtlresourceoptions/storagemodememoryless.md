# storageModeMemoryless

*Type Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 14.0, macOS 11.0, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodememoryless>

The resource’s contents are only available to the GPU, and only exist temporarily during a render pass.

## Declaration

```swift
static var storageModeMemoryless: MTLResourceOptions { get }
```

## Discussion

The memoryless storage mode uses tile memory, and is only available on Apple family GPUs. Memoryless resources are temporary targets used in a pass and you can’t access their contents with [MTLLoadAction.load](https://developer.apple.com/documentation/metal/mtlloadaction/load) or [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store).

Use memoryless resources for temporary elements used only within a single pass. For example, most render passes don’t store depth attachments and multisample attachments to memory. You can significantly reduce your memory usage by creating these attachments as memoryless resources.

On Metal devices that support tile rendering, you can use imageblocks to manage transient rendering data more flexibly. For more information about imageblock memory and using it with your shader functions, see the [Metal Shading Language Specification (PDF)](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf) sections 2.11, 4.5, and 5.6.

For more guidance on how to choose storage modes, see [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes).

## See also

### Specifying storage modes
- [storageModeShared](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeshared) — The CPU and GPU share access to the resource, allocated in system memory.
- [storageModeManaged](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodemanaged) — The CPU and GPU may maintain separate copies of the resource, and any changes need to be explicitly synchronized.
- [storageModePrivate](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeprivate) — The resource is only available to the GPU.
