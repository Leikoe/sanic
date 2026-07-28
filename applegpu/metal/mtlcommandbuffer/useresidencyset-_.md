# useResidencySet(_:)

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/useresidencyset(_:)>

Applies a residency set to a command buffer.

## Declaration

```swift
func useResidencySet(_ residencySet: any MTLResidencySet)
```

## Parameters

- **residencySet** — A residency set that contains resource allocations, such as [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances.

## Discussion

Each command buffer can maintain a list of up to 32 different residency sets. See [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) and [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) for more information.

## See also

### Attaching residency sets
- [useResidencySets(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/useresidencysets(_:)) — Applies multiple residency sets to a command buffer.
