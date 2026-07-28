# setPurgeableState(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresource/setpurgeablestate(_:)>

Specifies or queries the resource’s purgeable state.

## Declaration

```swift
func setPurgeableState(_ state: MTLPurgeableState) -> MTLPurgeableState
```

## Parameters

- **state** — The desired purgeable state of a resource.

## Return Value

The prior purgeable state of the resource.

## Discussion

If `state` is [MTLPurgeableState.keepCurrent](https://developer.apple.com/documentation/metal/mtlpurgeablestate/keepcurrent), the method returns the current purgeable state without changing it.

If `state` is [MTLPurgeableState.nonVolatile](https://developer.apple.com/documentation/metal/mtlpurgeablestate/nonvolatile), the resource is marked to inform the caller that the data should not be discarded.

If `state` is [MTLPurgeableState.empty](https://developer.apple.com/documentation/metal/mtlpurgeablestate/empty), the resource is marked as data that can be discarded, because the caller no longer needs the contents of the resource.

If `state` is [MTLPurgeableState.volatile](https://developer.apple.com/documentation/metal/mtlpurgeablestate/volatile), the resource is marked as data that can be discarded, even if the caller may need the resource. `MTLResource` objects can be made purgeable, even if the caller may need the resource, where the implementation can reclaim the underlying storage at any time without informing the app. Purgeable resources may enable an app to keep larger caches of idle memory that may be useful again in the future without the risk of preventing the allocation of more important memory.

When you use purgeable memory, make sure the block of memory is locked before you access it. This locking mechanism ensures that auto-removal policies don’t discard the data while you are accessing it. Similarly, the locking mechanism ensures that the virtual memory system has not already discarded the data.

## See also

### Setting the purgeable state of the resource
- [MTLPurgeableState](https://developer.apple.com/documentation/metal/mtlpurgeablestate) — The purgeable state of the resource.
