# hazardTrackingModeTracked

*Type Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourceoptions/hazardtrackingmodetracked>

An option that instructs Metal to apply safeguards for a resource at runtime to avoid memory hazards for the applicable commands.

## Declaration

```swift
static var hazardTrackingModeTracked: MTLResourceOptions { get }
```

## Discussion

For more information, see [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked).

## See also

### Specifying hazard tracking
- [hazardTrackingModeUntracked](https://developer.apple.com/documentation/metal/mtlresourceoptions/hazardtrackingmodeuntracked) — A resource option that instructs Metal to ignore memory hazards for a resource at runtime.
