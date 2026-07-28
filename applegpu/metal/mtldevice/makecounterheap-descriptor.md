# makeCounterHeap(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/makecounterheap(descriptor:)>

Creates a new counter heap configured from a counter heap descriptor.

## Declaration

```swift
func makeCounterHeap(descriptor: MTL4CounterHeapDescriptor) throws -> any MTL4CounterHeap
```

## Parameters

- **descriptor** — [MTL4CounterHeapDescriptor](https://developer.apple.com/documentation/metal/mtl4counterheapdescriptor) instance that configures the [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) instance.

## Return Value

A [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) instance, or `nil` if the function failed.
