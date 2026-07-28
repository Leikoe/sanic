# writeTimestamp(granularity:counterHeap:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writetimestamp(granularity:counterheap:index:)>

Writes a GPU timestamp into a heap.

## Declaration

```swift
func writeTimestamp(granularity: MTL4TimestampGranularity, counterHeap: any MTL4CounterHeap, index: Int)
```

## Parameters

- **granularity** — [MTL4TimestampGranularity](https://developer.apple.com/documentation/metal/mtl4timestampgranularity) hint to Metal about acceptable the level of precision.
- **counterHeap** — [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) to write timestamps into.
- **index** — The index value into which Metal writes the timestamp.

## Discussion

The method ensures that any prior work finishes, but doesn’t delay any subsequent work.

You can alter this command’s behavior through the `granularity` parameter.

- Pass [MTL4TimestampGranularity.relaxed](https://developer.apple.com/documentation/metal/mtl4timestampgranularity/relaxed) to allow Metal to provide timestamps with minimal impact to runtime performance, but with less detail. For example, the command may group all timestamps for a pass together.

- Pass [MTL4TimestampGranularity.precise](https://developer.apple.com/documentation/metal/mtl4timestampgranularity/precise) to request that Metal provides timestamps with the most detail. This can affect runtime performance.
