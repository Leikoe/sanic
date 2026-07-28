# MTLHazardTrackingMode.tracked

*Case · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked>

An option that directs Metal to apply runtime safeguards that prevent memory hazards when commands access a resource.

## Declaration

```swift
case tracked
```

## Discussion

Metal tracks memory dependencies for resources you create with this option. When at least one command writes to a tracked resource, the framework takes the following actions:

- Delay write operations until all previous read operations finish

- Prevent subsequent commands from running until write operations finish

This automatic hazard tracking provides safety for your resources without requiring you to manually synchronize access with barriers, fences, or events.

See [MTLHazardTrackingMode](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode) for more information about hazard tracking and how to enable it.

## See also

### Selecting the tracking mode
- [MTLHazardTrackingMode.default](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default) — An option that applies the default tracking behavior in Metal based on the resource or heap type you’re creating.
- [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) — An option that disables automatic memory hazard tracking in Metal for a resource at runtime.
