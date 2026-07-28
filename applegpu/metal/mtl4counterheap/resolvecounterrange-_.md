# resolveCounterRange(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4counterheap/resolvecounterrange(_:)>

Resolves heap data on the CPU timeline.

## Declaration

```swift
func resolveCounterRange(_ range: Range<Int>) throws -> Data?
```

## Parameters

- **range** — The range in the heap to resolve.

## Discussion

This method resolves heap data in the CPU timeline. Your app needs to ensure the GPU work has completed in order to retrieve the data correctly. You can alternatively resolve the heap data in the GPU timeline by calling `MTL4CommandBuffer/resolveCounterHeap:withRange:intoBuffer:atOffset:waitFence:updateFence:`.

- Returns a newly allocated autoreleased NSData containing tightly packed resolved heap counter values.

> **Note:**
> When resolving counters in the CPU timeline, signaling an instance of [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) after any workloads write counters (and waiting on that signal on the CPU) is sufficient to ensure synchronization.
