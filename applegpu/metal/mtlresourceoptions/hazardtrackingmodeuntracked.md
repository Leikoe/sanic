# hazardTrackingModeUntracked

*Type Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourceoptions/hazardtrackingmodeuntracked>

A resource option that instructs Metal to ignore memory hazards for a resource at runtime.

## Declaration

```swift
static var hazardTrackingModeUntracked: MTLResourceOptions { get }
```

## Discussion

For more information, see [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked).

## See also

### Specifying hazard tracking
- [hazardTrackingModeTracked](https://developer.apple.com/documentation/metal/mtlresourceoptions/hazardtrackingmodetracked) — An option that instructs Metal to apply safeguards for a resource at runtime to avoid memory hazards for the applicable commands.
