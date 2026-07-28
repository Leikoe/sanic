# MTLSamplePositionMake(_:_:)

*Function · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplepositionmake(_:_:)>

Returns a new sample position on a subpixel grid.

## Declaration

```swift
func MTLSamplePositionMake(_ x: Float, _ y: Float) -> MTLSamplePosition
```

## Parameters

- **x** — The x coordinate.
- **y** — The y coordinate.

## Return Value

The new sample position.

## See also

### Using programmable sample positions
- [setSamplePositions(_:)](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/setsamplepositions(_:)) — Sets the programmable sample positions for a render pass.
- [getSamplePositions()](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/getsamplepositions()) — Returns the programmable sample positions set for a render pass.
