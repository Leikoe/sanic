# MTLStorageMode.private

*Case · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstoragemode/private>

The resource is only available to the GPU.

## Declaration

```swift
case `private`
```

## Discussion

Metal may apply additional optimizations to private resources that aren’t allowed on shared or managed resources.

For more guidance on how to choose storage modes, see [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes).

## See also

### Storage mode options
- [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) — The CPU and GPU share access to the resource, allocated in system memory.
- [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed) — The CPU and GPU may maintain separate copies of the resource, and any changes need to be explicitly synchronized.
- [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless) — The resource’s contents are only available to the GPU, and only exist temporarily during a render pass.
