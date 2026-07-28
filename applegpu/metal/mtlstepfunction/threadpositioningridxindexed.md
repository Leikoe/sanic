# MTLStepFunction.threadPositionInGridXIndexed

*Case · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstepfunction/threadpositioningridxindexed>

The compute function fetches data by using the thread’s `x` coordinate to look up a value in the index buffer.

## Declaration

```swift
case threadPositionInGridXIndexed
```

## Discussion

This step function uses the `x` coordinate of the thread position in a grid as an index into the `[[stage_in]]` index buffer, which is then used to fetch data. In tessellation compute kernels, you use this step function to identify a control point in a given patch.

## See also

### Step options
- [MTLStepFunction.constant](https://developer.apple.com/documentation/metal/mtlstepfunction/constant) — The function fetches attribute data once.
- [MTLStepFunction.perInstance](https://developer.apple.com/documentation/metal/mtlstepfunction/perinstance) — The function fetches data based on the instance index.
- [MTLStepFunction.perPatch](https://developer.apple.com/documentation/metal/mtlstepfunction/perpatch) — The post-tessellation function fetches data based on the patch index of the patch.
- [MTLStepFunction.perPatchControlPoint](https://developer.apple.com/documentation/metal/mtlstepfunction/perpatchcontrolpoint) — The post-tessellation function fetches data based on the control-point indices associated with the patch.
- [MTLStepFunction.perVertex](https://developer.apple.com/documentation/metal/mtlstepfunction/pervertex) — The vertex function fetches data for every vertex.
- [MTLStepFunction.threadPositionInGridX](https://developer.apple.com/documentation/metal/mtlstepfunction/threadpositioningridx) — The compute function fetches data based on the thread’s `x` coordinate.
- [MTLStepFunction.threadPositionInGridY](https://developer.apple.com/documentation/metal/mtlstepfunction/threadpositioningridy) — The compute function fetches data based on the thread’s `y` coordinate.
- [MTLStepFunction.threadPositionInGridYIndexed](https://developer.apple.com/documentation/metal/mtlstepfunction/threadpositioningridyindexed) — The compute function fetches data by using the thread’s `y` coordinate to look up a value in the index buffer.
