# size(ofCounterHeapEntry:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/size(ofcounterheapentry:)>

Returns the size, in bytes, of each entry in a counter heap of a specific counter heap type when your app resolves it into a usable format.

## Declaration

```swift
func size(ofCounterHeapEntry type: MTL4CounterHeapType) -> Int
```

## Parameters

- **type** — [MTL4CounterHeapType](https://developer.apple.com/documentation/metal/mtl4counterheaptype) value that represents the type of the [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) to resolve.

## Return Value

The size of the post-transformation entry from a [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) of type [MTL4CounterHeapType](https://developer.apple.com/documentation/metal/mtl4counterheaptype).

## Discussion

In order to use the data available in a [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap), your app first resolves it either in the CPU timeline or in the GPU timeline. When your app calls [resolveCounterHeap:withRange:intoBuffer:waitFence:updateFence:](https://developer.apple.com/documentation/metal/mtl4commandbuffer/resolvecounterheap:withrange:intobuffer:waitfence:updatefence:) to resolve counter data in the GPU timeline, Metal writes the data into a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).

During this process, Metal transform the data in the heap into a format consisting of entries of the size that this method advertises, based on the [MTL4CounterHeapType](https://developer.apple.com/documentation/metal/mtl4counterheaptype).
