# intermediatesHeapSize

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinestate/intermediatesheapsize>

Obtain the size of the heap, in bytes, this pipeline requires during the execution.

## Declaration

```swift
var intermediatesHeapSize: Int { get }
```

## Discussion

Use this value to allocate a [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instance of sufficient size that you can then provide to [dispatchNetwork(intermediatesHeap:)](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/dispatchnetwork(intermediatesheap:)).

Metal uses this heap to store intermediate data as it executes the pipeline. It is your responsibility to provide a heap at least as large as this property requests.
