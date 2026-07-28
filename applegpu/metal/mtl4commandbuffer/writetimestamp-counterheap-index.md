# writeTimestamp(counterHeap:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandbuffer/writetimestamp(counterheap:index:)>

Writes a GPU timestamp into the given counter heap.

## Declaration

```swift
func writeTimestamp(counterHeap: any MTL4CounterHeap, index: Int)
```

## Parameters

- **counterHeap** — [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) to write the timestamp into.
- **index** — The index within the [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) that Metal writes the timestamp to.

## Discussion

This method captures a timestamp after work prior to this command in the command buffer is complete. Work after this call may or may not have started.

You are responsible for ensuring the `counterHeap` is of type [MTL4CounterHeapType.timestamp](https://developer.apple.com/documentation/metal/mtl4counterheaptype/timestamp).
