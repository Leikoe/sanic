# MTLTessellationFactorStepFunction.perPatch

*Case · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/perpatch>

A per-patch step function. For all instances, the tessellation factor for all patches in a patch draw call is at the `offset + (drawPatchIndex * tessellationFactorStride)` location in the tessellation factor buffer.

## Declaration

```swift
case perPatch
```

## See also

### Factor step functions
- [MTLTessellationFactorStepFunction.constant](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/constant) — A constant step function. For all instances, the tessellation factor for all patches in a patch draw call is at the `offset` location in the tessellation factor buffer.
- [MTLTessellationFactorStepFunction.perInstance](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/perinstance) — A per-instance step function. For a given instance ID, the tessellation factor for a patch in a patch draw call is at the `offset + (instanceID * instanceStride)` location in the tessellation factor buffer.
- [MTLTessellationFactorStepFunction.perPatchAndPerInstance](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/perpatchandperinstance) — A per-patch and per-instance step function. For a given instance ID, the tessellation factor for a patch in a patch draw call is at the `offset + (drawPatchIndex * tessellationFactorStride + instanceID * instanceStride)` location in the tessellation factor buffer.
