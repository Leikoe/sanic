# MTLHeapType.placement

*Case · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheaptype/placement>

The app controls placement of resources on the heap.

## Declaration

```swift
case placement
```

## Discussion

Use placement heaps when you need direct control over memory use and heap fragmentation. Typically, you use placement heaps for resources you keep for long time periods and rarely change.

## See also

### Specifying the heap type
- [MTLHeapType.automatic](https://developer.apple.com/documentation/metal/mtlheaptype/automatic) — A heap that automatically places new resource allocations.
- [MTLHeapType.sparse](https://developer.apple.com/documentation/metal/mtlheaptype/sparse) — The heap contains sparse texture tiles.
