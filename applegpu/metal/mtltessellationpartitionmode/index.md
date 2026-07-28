# MTLTessellationPartitionMode

*Enumeration · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltessellationpartitionmode>

Options for choosing the partition mode that the tessellator applies when deriving the number and spacing of segments for subdividing a corresponding edge.

## Declaration

```swift
enum MTLTessellationPartitionMode
```

## Overview

The table lists the tessellation factor range for each partitioning mode.

| Partitioning mode | Tessellation factor range |
|---|---|
| [MTLTessellationPartitionMode.pow2](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/pow2) | [`1`, [maxTessellationFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxtessellationfactor)] |
| [MTLTessellationPartitionMode.integer](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/integer) | [`1`, [maxTessellationFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxtessellationfactor)] |
| [MTLTessellationPartitionMode.fractionalOdd](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/fractionalodd) | [`1`, [maxTessellationFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxtessellationfactor)-1] |
| [MTLTessellationPartitionMode.fractionalEven](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/fractionaleven) | [`2`, [maxTessellationFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxtessellationfactor)] |

The floating-point tessellation level is always clamped to its corresponding range before calculating the final tessellation factor. After clamping, the calculation depends on the chosen partitioning mode:

- For the [MTLTessellationPartitionMode.pow2](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/pow2) partitioning mode, the result is rounded up to the nearest integer `n`, where `n` is a power of two. The corresponding edge is divided into `n` segments of equal length in (u, v) space.

- For the [MTLTessellationPartitionMode.integer](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/integer) partitioning mode, the result is rounded up to the nearest integer `n`. The corresponding edge is divided into `n` segments of equal length in (u, v) space.

- For the [MTLTessellationPartitionMode.fractionalOdd](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/fractionalodd) partitioning mode, the tessellation level is rounded up the the nearest odd integer `n`. If `n` is `1`, the edge is not subdivided. Otherwise, the corresponding edge is divided into `n-2` segments of equal length, and two additional segments of equal length that are typically shorter than the other segments. The length of the two additional segments relative to the others decreases monotonically by the value of `n-f`, where `f` is the clamped floating-point tessellation level. If `n-f` is `0` the additional segments equal length to the other segments. As `n-f` approaches `2`, the relative length of the additional segments approaches `0`. The two additional segments should be placed symmetrically on opposite sides of the subdivided edge. The relative location of these two segments is undefined, but needs to be identical for any pair of subdivided edges with identical values of `f`.

- For the [MTLTessellationPartitionMode.fractionalEven](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/fractionaleven) partitioning mode, the tessellation level is rounded up the the nearest even integer `n`.

## Topics

### Partition modes
- [MTLTessellationPartitionMode.pow2](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/pow2) — A power of two partitioning mode.
- [MTLTessellationPartitionMode.integer](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/integer) — An integer partitioning mode.
- [MTLTessellationPartitionMode.fractionalOdd](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/fractionalodd) — A fractional odd partitioning mode.
- [MTLTessellationPartitionMode.fractionalEven](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/fractionaleven) — A fractional even partitioning mode.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode/init(rawvalue:))

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
- [MTLTessellationFactorStepFunction](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction) — Options for specifying the step function that determines the tessellation factors for a patch from the tessellation factor buffer.
