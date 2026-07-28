# extendedLimits

*Type Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/extendedlimits>

An option that increases an acceleration structure’s storage capacity.

## Declaration

```swift
static var extendedLimits: MTLAccelerationStructureUsage { get }
```

## Discussion

The acceleration structures you build with this option can affect their performance because they support more data complexity.

|  | Standard limits | Extended limits |
|---|---|---|
| Primitives in primitive acceleration structure | `2^(28)` | `2^(30)` |
| Geometries in primitive acceleration structure | `2^(24)` | `2^(30)` |
| Instances in instance acceleration structure | `2^(24)` | `2^(30)` |
| Visibility mask bits | `8` | `32` |

## See also

### Applying options
- [refit](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/refit) — An option that lets you update an acceleration structure after creating it.
- [preferFastBuild](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastbuild) — An option that instructs Metal to build an acceleration structure quickly.
- [preferFastIntersection](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastintersection) — An option that instructs Metal to prioritize building an acceleration structure with better intersection performance.
- [minimizeMemory](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/minimizememory) — An option that instructs Metal to prioritize building an acceleration structure that needs less memory.
