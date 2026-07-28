# preferFastBuild

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastbuild>

An option that instructs Metal to build an acceleration structure quickly.

## Declaration

```swift
static var preferFastBuild: MTLAccelerationStructureUsage { get }
```

## Discussion

Apply this option when you need to reduce the time when creating or refitting an acceleration structure, such as from code that’s sensitive to runtime performance.

> **Note:**
>  The acceleration structures you build with this option can reduce their intersection performance.

## See also

### Applying options
- [refit](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/refit) — An option that lets you update an acceleration structure after creating it.
- [preferFastIntersection](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastintersection) — An option that instructs Metal to prioritize building an acceleration structure with better intersection performance.
- [minimizeMemory](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/minimizememory) — An option that instructs Metal to prioritize building an acceleration structure that needs less memory.
- [extendedLimits](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/extendedlimits) — An option that increases an acceleration structure’s storage capacity.
