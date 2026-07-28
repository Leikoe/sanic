# tessellationFactorStepFunction

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationfactorstepfunction>

The step function for determining the tessellation factors for a patch from the tessellation factor buffer.

## Declaration

```swift
var tessellationFactorStepFunction: MTLTessellationFactorStepFunction { get set }
```

## Discussion

The default value is [MTLTessellationFactorStepFunction.constant](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction/constant).

## See also

### Specifying tessellation state
- [maxTessellationFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxtessellationfactor) — The maximum tessellation factor that the tessellator uses when tessellating patches.
- [isTessellationFactorScaleEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/istessellationfactorscaleenabled) — A Boolean value that determines whether the pipeline scales the tessellation factor.
- [tessellationFactorFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationfactorformat) — The format of the tessellation factors in the tessellation factor buffer.
- [tessellationControlPointIndexType](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationcontrolpointindextype) — The size of the control point indices in a control point index buffer.
- [tessellationOutputWindingOrder](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationoutputwindingorder) — The winding order of triangles from the tessellator.
- [tessellationPartitionMode](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationpartitionmode) — The partitioning mode that the tessellator uses to derive the number and spacing of segments for subdividing a corresponding edge.
- [MTLTessellationFactorFormat](https://developer.apple.com/documentation/metal/mtltessellationfactorformat) — Options for specifying the format of the tessellation factors in a tessellation factor buffer.
- [MTLTessellationControlPointIndexType](https://developer.apple.com/documentation/metal/mtltessellationcontrolpointindextype) — Options for specifying the size of the control point indices in a control point index buffer.
- [MTLTessellationFactorStepFunction](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction) — Options for specifying the step function that determines the tessellation factors for a patch from the tessellation factor buffer.
- [MTLTessellationPartitionMode](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode) — Options for choosing the partition mode that the tessellator applies when deriving the number and spacing of segments for subdividing a corresponding edge.
