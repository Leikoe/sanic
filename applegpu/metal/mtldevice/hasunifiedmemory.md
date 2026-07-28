# hasUnifiedMemory

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/hasunifiedmemory>

A Boolean value that indicates whether the GPU shares all of its memory with the CPU.

## Declaration

```swift
var hasUnifiedMemory: Bool { get }
```

## Discussion

A GPU with unified memory ([true](https://developer.apple.com/documentation/Swift/true)) is typically an integrated GPU. A GPU with dedicated memory ([false](https://developer.apple.com/documentation/Swift/false)) may take additional time to synchronize managed resources or copy data into private GPU resources.

## See also

### Checking a GPU device’s memory
- [currentAllocatedSize](https://developer.apple.com/documentation/metal/mtldevice/currentallocatedsize) — The total amount of memory, in bytes, the GPU device is using for all of its resources.
- [recommendedMaxWorkingSetSize](https://developer.apple.com/documentation/metal/mtldevice/recommendedmaxworkingsetsize) — An approximation of how much memory, in bytes, this GPU device can allocate without affecting its runtime performance.
- [maxTransferRate](https://developer.apple.com/documentation/metal/mtldevice/maxtransferrate) — The highest theoretical rate, in bytes per second, the system can copy between system memory and the GPU’s dedicated memory (VRAM).
