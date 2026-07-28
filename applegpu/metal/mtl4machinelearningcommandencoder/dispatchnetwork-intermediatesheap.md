# dispatchNetwork(intermediatesHeap:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/dispatchnetwork(intermediatesheap:)>

Dispatches a machine learning network using the current pipeline state and argument table.

## Declaration

```swift
func dispatchNetwork(intermediatesHeap heap: any MTLHeap)
```

## Parameters

- **heap** — A heap that Metal can use to allocate intermediate tensors.

## Discussion

This method takes a parameter consisting of a `MTLHeap` that Metal can use to allocate intermediate tensors. You can query the minimum size Metal requires for this heap by calling [intermediatesHeapSize](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinestate/intermediatesheapsize).
