# maxTransferRate

*Instance Property · macOS 10.15*

<https://developer.apple.com/documentation/metal/mtldevice/maxtransferrate>

The highest theoretical rate, in bytes per second, the system can copy between system memory and the GPU’s dedicated memory (VRAM).

## Declaration

```swift
var maxTransferRate: UInt64 { get }
```

## Discussion

Metal calculates this value from the raw data-clock rate, and the GPU may not be able to reach this speed in real-world conditions.

> **Important:**
>  The maximum transfer rate for built-in GPUs is `0`.

## See also

### Checking a GPU device’s memory
- [currentAllocatedSize](https://developer.apple.com/documentation/metal/mtldevice/currentallocatedsize) — The total amount of memory, in bytes, the GPU device is using for all of its resources.
- [recommendedMaxWorkingSetSize](https://developer.apple.com/documentation/metal/mtldevice/recommendedmaxworkingsetsize) — An approximation of how much memory, in bytes, this GPU device can allocate without affecting its runtime performance.
- [hasUnifiedMemory](https://developer.apple.com/documentation/metal/mtldevice/hasunifiedmemory) — A Boolean value that indicates whether the GPU shares all of its memory with the CPU.
