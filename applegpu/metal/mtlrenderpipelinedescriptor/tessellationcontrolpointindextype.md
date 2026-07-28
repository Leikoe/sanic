# tessellationControlPointIndexType

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationcontrolpointindextype>

The size of the control point indices in a control point index buffer.

## Declaration

```swift
var tessellationControlPointIndexType: MTLTessellationControlPointIndexType { get set }
```

## Discussion

The default value is [MTLTessellationControlPointIndexType.none](https://developer.apple.com/documentation/metal/mtltessellationcontrolpointindextype/none); use this value when drawing patches without a control point index buffer. This value needs to be either [MTLTessellationControlPointIndexType.uint16](https://developer.apple.com/documentation/metal/mtltessellationcontrolpointindextype/uint16) or [MTLTessellationControlPointIndexType.uint32](https://developer.apple.com/documentation/metal/mtltessellationcontrolpointindextype/uint32) when drawing patches with indexed control points.

## See also

### Related Documentation
- [drawIndexedPatches(numberOfPatchControlPoints:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedpatches(numberofpatchcontrolpoints:patchstart:patchcount:patchindexbuffer:patchindexbufferoffset:controlpointindexbuffer:controlpointindexbufferoffset:instancecount:baseinstance:)) — Encodes a draw command that renders multiple instances of tessellated patches with a control point index buffer.
- [drawIndexedPatches(numberOfPatchControlPoints:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedpatches(numberofpatchcontrolpoints:patchindexbuffer:patchindexbufferoffset:controlpointindexbuffer:controlpointindexbufferoffset:indirectbuffer:indirectbufferoffset:)) — Encodes a draw command that renders multiple instances of tessellated patches with a control point index buffer and indirect arguments.

### Specifying tessellation state
- [maxTessellationFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxtessellationfactor) — The maximum tessellation factor that the tessellator uses when tessellating patches.
- [isTessellationFactorScaleEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/istessellationfactorscaleenabled) — A Boolean value that determines whether the pipeline scales the tessellation factor.
- [tessellationFactorFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationfactorformat) — The format of the tessellation factors in the tessellation factor buffer.
- [tessellationFactorStepFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationfactorstepfunction) — The step function for determining the tessellation factors for a patch from the tessellation factor buffer.
- [tessellationOutputWindingOrder](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationoutputwindingorder) — The winding order of triangles from the tessellator.
- [tessellationPartitionMode](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationpartitionmode) — The partitioning mode that the tessellator uses to derive the number and spacing of segments for subdividing a corresponding edge.
- [MTLTessellationFactorFormat](https://developer.apple.com/documentation/metal/mtltessellationfactorformat) — Options for specifying the format of the tessellation factors in a tessellation factor buffer.
- [MTLTessellationControlPointIndexType](https://developer.apple.com/documentation/metal/mtltessellationcontrolpointindextype) — Options for specifying the size of the control point indices in a control point index buffer.
- [MTLTessellationFactorStepFunction](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction) — Options for specifying the step function that determines the tessellation factors for a patch from the tessellation factor buffer.
- [MTLTessellationPartitionMode](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode) — Options for choosing the partition mode that the tessellator applies when deriving the number and spacing of segments for subdividing a corresponding edge.
