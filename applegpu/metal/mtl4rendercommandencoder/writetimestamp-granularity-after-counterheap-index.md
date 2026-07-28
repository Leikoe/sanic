# writeTimestamp(granularity:after:counterHeap:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/writetimestamp(granularity:after:counterheap:index:)>

Writes a GPU timestamp into the given [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) at `index` after `stage` completes.

## Declaration

```swift
func writeTimestamp(granularity: MTL4TimestampGranularity, after stage: MTLRenderStages, counterHeap: any MTL4CounterHeap, index: Int)
```

## Parameters

- **granularity** — A [MTL4TimestampGranularity](https://developer.apple.com/documentation/metal/mtl4timestampgranularity) hint.
- **stage** — [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) that need to complete before Metal writes the timestamp. This may also include later stages that are related, for example [mesh](https://developer.apple.com/documentation/metal/mtlrenderstages/mesh) may include [vertex](https://developer.apple.com/documentation/metal/mtlrenderstages/vertex).
- **counterHeap** — [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) into which Metal writes timestamps.
- **index** — The index value into which Metal writes this timestamp.

## Discussion

This command only guarantees all draws prior to this command are complete when Metal writes the timestamp into the counter heap you provide in the `counterHeap` parameter. The timestamp may also include subsequent operations.

If you call this method before any draw calls, Metal writes a timestamp before the stage you specify in the `stage` parameter begins.
