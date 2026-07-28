# isAliasable()

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresource/isaliasable()>

A Boolean value that indicates whether future heap resource allocations may alias against the resource’s memory.

## Declaration

```swift
func isAliasable() -> Bool
```

## Return Value

The default value is [false](https://developer.apple.com/documentation/Swift/false). The value is [true](https://developer.apple.com/documentation/Swift/true) only if the [makeAliasable()](https://developer.apple.com/documentation/metal/mtlresource/makealiasable()) method was previously called on this resource.

## See also

### Managing heap resources
- [heapOffset](https://developer.apple.com/documentation/metal/mtlresource/heapoffset) — The distance, in bytes, from the beginning of the heap to the first byte of the resource, if you allocated the resource on a heap.
- [heap](https://developer.apple.com/documentation/metal/mtlresource/heap) — The heap on which the resource is allocated, if any.
- [makeAliasable()](https://developer.apple.com/documentation/metal/mtlresource/makealiasable()) — Allows future heap resource allocations to alias against the resource’s memory, reusing it.
