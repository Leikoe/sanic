# MTLTessellationFactorStepFunction

*Enumeration · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction>

Options for specifying the step function that determines the tessellation factors for a patch from the tessellation factor buffer.

## Declaration

```swift
enum MTLTessellationFactorStepFunction
```

## Topics

### Factor step functions
- [MTLTessellationFactorStepFunction.constant](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/constant) — A constant step function. For all instances, the tessellation factor for all patches in a patch draw call is at the `offset` location in the tessellation factor buffer.
- [MTLTessellationFactorStepFunction.perPatch](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/perpatch) — A per-patch step function. For all instances, the tessellation factor for all patches in a patch draw call is at the `offset + (drawPatchIndex * tessellationFactorStride)` location in the tessellation factor buffer.
- [MTLTessellationFactorStepFunction.perInstance](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/perinstance) — A per-instance step function. For a given instance ID, the tessellation factor for a patch in a patch draw call is at the `offset + (instanceID * instanceStride)` location in the tessellation factor buffer.
- [MTLTessellationFactorStepFunction.perPatchAndPerInstance](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/perpatchandperinstance) — A per-patch and per-instance step function. For a given instance ID, the tessellation factor for a patch in a patch draw call is at the `offset + (drawPatchIndex * tessellationFactorStride + instanceID * instanceStride)` location in the tessellation factor buffer.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/init(rawvalue:))

## See also

### Specifying tessellation state
- [maxTessellationFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxtessellationfactor) — The maximum tessellation factor that the tessellator uses when tessellating patches.
- [isTessellationFactorScaleEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/istessellationfactorscaleenabled) — A Boolean value that determines whether the pipeline scales the tessellation factor.
- [tessellationFactorFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationfactorformat) — The format of the tessellation factors in the tessellation factor buffer.
- [tessellationControlPointIndexType](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationcontrolpointindextype) — The size of the control point indices in a control point index buffer.
- [tessellationFactorStepFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationfactorstepfunction) — The step function for determining the tessellation factors for a patch from the tessellation factor buffer.
- [tessellationOutputWindingOrder](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationoutputwindingorder) — The winding order of triangles from the tessellator.
- [tessellationPartitionMode](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationpartitionmode) — The partitioning mode that the tessellator uses to derive the number and spacing of segments for subdividing a corresponding edge.
- [MTLTessellationFactorFormat](https://developer.apple.com/documentation/metal/mtltessellationfactorformat) — Options for specifying the format of the tessellation factors in a tessellation factor buffer.
- [MTLTessellationControlPointIndexType](https://developer.apple.com/documentation/metal/mtltessellationcontrolpointindextype) — Options for specifying the size of the control point indices in a control point index buffer.
- [MTLTessellationPartitionMode](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode) — Options for choosing the partition mode that the tessellator applies when deriving the number and spacing of segments for subdividing a corresponding edge.
