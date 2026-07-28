# MTLHazardTrackingMode.default

*Case · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default>

An option that applies the default tracking behavior in Metal based on the resource or heap type you’re creating.

## Declaration

```swift
case `default`
```

## Discussion

When you choose the [MTLHazardTrackingMode.default](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default) option, Metal assigns a tracking mode based on the type you’re creating:

- The default tracking mode for an [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) is [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) because heaps typically contain many resources that you manage manually.

- The default tracking mode for a type that inherits [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) is [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked) because individual resources benefit from automatic hazard tracking.

For example, Metal tracks hazards for [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) and [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instances when you create them with [MTLHazardTrackingMode.default](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default).

For more information, see [MTLHazardTrackingMode](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode).

## See also

### Selecting the tracking mode
- [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) — An option that disables automatic memory hazard tracking in Metal for a resource at runtime.
- [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked) — An option that directs Metal to apply runtime safeguards that prevent memory hazards when commands access a resource.
