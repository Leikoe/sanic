# heapOffset

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresource/heapoffset>

The distance, in bytes, from the beginning of the heap to the first byte of the resource, if you allocated the resource on a heap.

## Declaration

```swift
var heapOffset: Int { get }
```

## Discussion

If the heap is not a placement heap ([MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement)), the value is always `0` and should be ignored.

## See also

### Managing heap resources
- [heap](https://developer.apple.com/documentation/metal/mtlresource/heap) — The heap on which the resource is allocated, if any.
- [makeAliasable()](https://developer.apple.com/documentation/metal/mtlresource/makealiasable()) — Allows future heap resource allocations to alias against the resource’s memory, reusing it.
- [isAliasable()](https://developer.apple.com/documentation/metal/mtlresource/isaliasable()) — A Boolean value that indicates whether future heap resource allocations may alias against the resource’s memory.
