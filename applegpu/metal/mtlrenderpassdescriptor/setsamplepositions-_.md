# setSamplePositions(_:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 11.0, macOS 10.13, tvOS 11.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/setsamplepositions(_:)>

Sets the programmable sample positions for a render pass.

## Declaration

```swift
func setSamplePositions(_ positions: [MTLSamplePosition])
```

## Parameters

- **positions** — An array of programmable sample positions for the render pass with the the same number of elements as the render pass sample count, or an empty array to disable custom sample positions.

## Discussion

Programmable sample positions need to be floating-point values in the `[0.0, 1.0)` range along each axis, with the origin `(0,0)` defined at the top-left corner. Values can be set from `0/16` up to `15/16`, inclusive, in `1/16` increments along each axis.

If the length of the array is `0`, the GPU uses the default sample positions for the render pass.

> **Note:**
>  Call the [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) method to determine whether the device object supports a specific sample count.

## See also

### Using programmable sample positions
- [MTLSamplePositionMake(_:_:)](https://developer.apple.com/documentation/metal/mtlsamplepositionmake(_:_:)) — Returns a new sample position on a subpixel grid.
- [getSamplePositions()](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/getsamplepositions()) — Returns the programmable sample positions set for a render pass.
