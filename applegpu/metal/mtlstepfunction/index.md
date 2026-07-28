# MTLStepFunction

*Enumeration · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstepfunction>

The frequency and locations at which a function fetches attribute data.

## Declaration

```swift
enum MTLStepFunction
```

## Topics

### Step options
- [MTLStepFunction.constant](https://developer.apple.com/documentation/metal/mtlstepfunction/constant) — The function fetches attribute data once.
- [MTLStepFunction.perInstance](https://developer.apple.com/documentation/metal/mtlstepfunction/perinstance) — The function fetches data based on the instance index.
- [MTLStepFunction.perPatch](https://developer.apple.com/documentation/metal/mtlstepfunction/perpatch) — The post-tessellation function fetches data based on the patch index of the patch.
- [MTLStepFunction.perPatchControlPoint](https://developer.apple.com/documentation/metal/mtlstepfunction/perpatchcontrolpoint) — The post-tessellation function fetches data based on the control-point indices associated with the patch.
- [MTLStepFunction.perVertex](https://developer.apple.com/documentation/metal/mtlstepfunction/pervertex) — The vertex function fetches data for every vertex.
- [MTLStepFunction.threadPositionInGridX](https://developer.apple.com/documentation/metal/mtlstepfunction/threadpositioningridx) — The compute function fetches data based on the thread’s `x` coordinate.
- [MTLStepFunction.threadPositionInGridY](https://developer.apple.com/documentation/metal/mtlstepfunction/threadpositioningridy) — The compute function fetches data based on the thread’s `y` coordinate.
- [MTLStepFunction.threadPositionInGridXIndexed](https://developer.apple.com/documentation/metal/mtlstepfunction/threadpositioningridxindexed) — The compute function fetches data by using the thread’s `x` coordinate to look up a value in the index buffer.
- [MTLStepFunction.threadPositionInGridYIndexed](https://developer.apple.com/documentation/metal/mtlstepfunction/threadpositioningridyindexed) — The compute function fetches data by using the thread’s `y` coordinate to look up a value in the index buffer.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlstepfunction/init(rawvalue:))

## See also

### Describing fetch behavior
- [stride](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/stride) — The number of bytes from one buffer entry to the next.
- [stepFunction](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/stepfunction) — Determines how and when compute functions fetch data.
- [stepRate](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor/steprate) — How frequently the step function should load data.
