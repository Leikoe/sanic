# refit

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/refit>

An option that lets you update an acceleration structure after creating it.

## Declaration

```swift
static var refit: MTLAccelerationStructureUsage { get }
```

## Discussion

Apply this option to make a modifiable acceleration structure, which you can update over time, such as for geometry changes. By default, the framework builds immutable acceleration structures for performance. When you apply the [refit](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/refit) option, the framework builds an acceleration structure more conservatively, which can reduce its intersection performance.

> **Note:**
>  Refitting an acceleration structure generally works better when the geometry changes are relatively small.

## See also

### Applying options
- [preferFastBuild](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastbuild) — An option that instructs Metal to build an acceleration structure quickly.
- [preferFastIntersection](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastintersection) — An option that instructs Metal to prioritize building an acceleration structure with better intersection performance.
- [minimizeMemory](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/minimizememory) — An option that instructs Metal to prioritize building an acceleration structure that needs less memory.
- [extendedLimits](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/extendedlimits) — An option that increases an acceleration structure’s storage capacity.
