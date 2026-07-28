# minimizeMemory

*Type Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/minimizememory>

An option that instructs Metal to prioritize building an acceleration structure that needs less memory.

## Declaration

```swift
static var minimizeMemory: MTLAccelerationStructureUsage { get }
```

## Discussion

The acceleration structures you build with this option can increase their build times and can reduce their intersection performance.

## See also

### Applying options
- [refit](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/refit) — An option that lets you update an acceleration structure after creating it.
- [preferFastBuild](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastbuild) — An option that instructs Metal to build an acceleration structure quickly.
- [preferFastIntersection](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastintersection) — An option that instructs Metal to prioritize building an acceleration structure with better intersection performance.
- [extendedLimits](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/extendedlimits) — An option that increases an acceleration structure’s storage capacity.
