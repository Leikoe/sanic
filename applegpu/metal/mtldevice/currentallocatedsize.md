# currentAllocatedSize

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/currentallocatedsize>

The total amount of memory, in bytes, the GPU device is using for all of its resources.

## Declaration

```swift
var currentAllocatedSize: Int { get }
```

## See also

### Checking a GPU device’s memory
- [recommendedMaxWorkingSetSize](https://developer.apple.com/documentation/metal/mtldevice/recommendedmaxworkingsetsize) — An approximation of how much memory, in bytes, this GPU device can allocate without affecting its runtime performance.
- [hasUnifiedMemory](https://developer.apple.com/documentation/metal/mtldevice/hasunifiedmemory) — A Boolean value that indicates whether the GPU shares all of its memory with the CPU.
- [maxTransferRate](https://developer.apple.com/documentation/metal/mtldevice/maxtransferrate) — The highest theoretical rate, in bytes per second, the system can copy between system memory and the GPU’s dedicated memory (VRAM).
