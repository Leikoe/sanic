# MTLResourceOptions

*Structure · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourceoptions>

Optional arguments used to set the behavior of a resource.

## Declaration

```swift
struct MTLResourceOptions
```

## Topics

### Initializing resource options
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlresourceoptions/init(rawvalue:))

### Specifying CPU cache modes
- [cpuCacheModeWriteCombined](https://developer.apple.com/documentation/metal/mtlresourceoptions/cpucachemodewritecombined) — A write-combined CPU cache mode that is optimized for resources that the CPU writes into, but never reads.

### Specifying storage modes
- [storageModeShared](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeshared) — The CPU and GPU share access to the resource, allocated in system memory.
- [storageModeManaged](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodemanaged) — The CPU and GPU may maintain separate copies of the resource, and any changes need to be explicitly synchronized.
- [storageModePrivate](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeprivate) — The resource is only available to the GPU.
- [storageModeMemoryless](https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodememoryless) — The resource’s contents are only available to the GPU, and only exist temporarily during a render pass.

### Specifying hazard tracking
- [hazardTrackingModeTracked](https://developer.apple.com/documentation/metal/mtlresourceoptions/hazardtrackingmodetracked) — An option that instructs Metal to apply safeguards for a resource at runtime to avoid memory hazards for the applicable commands.
- [hazardTrackingModeUntracked](https://developer.apple.com/documentation/metal/mtlresourceoptions/hazardtrackingmodeuntracked) — A resource option that instructs Metal to ignore memory hazards for a resource at runtime.

### Deprecated options
- [optionCPUCacheModeWriteCombined](https://developer.apple.com/documentation/metal/mtlresourceoptions/optioncpucachemodewritecombined) — This constant was deprecated in iOS 9.0 and macOS 10.11.

## See also

### Common resource functionality
- [MTLGPUAddress](https://developer.apple.com/documentation/metal/mtlgpuaddress) — A 64-bit unsigned integer type appropriate for storing GPU addresses.
- [MTLAllocation](https://developer.apple.com/documentation/metal/mtlallocation) — A memory allocation from a Metal GPU device, such as a memory heap, texture, or data buffer.
- [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) — An allocation of memory accessible to a GPU.
- [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) — Options that describe how a graphics or compute function uses an argument buffer’s resource.
- [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid)
