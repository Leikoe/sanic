# supportsVertexAmplificationCount(_:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportsvertexamplificationcount(_:)>

Returns a Boolean value that indicates whether the GPU supports an amplification factor.

## Declaration

```swift
func supportsVertexAmplificationCount(_ count: Int) -> Bool
```

## Parameters

- **count** — An integer that represents the number of output streams you want the GPU to generate from an input stream.

## Discussion

A vertex amplification factor of `1` has no effect because it effectively disables vertex amplification.

> **Important:**
>  Passing a vertex amplification factor of `1` or less to this method triggers an API validation error.

For more information about vertex amplification, see [Improving rendering performance with vertex amplification](https://developer.apple.com/documentation/metal/improving-rendering-performance-with-vertex-amplification).

## See also

### Checking render support
- [supportsRaytracing](https://developer.apple.com/documentation/metal/mtldevice/supportsraytracing) — A Boolean value that indicates whether the GPU device supports ray tracing.
- [supportsPrimitiveMotionBlur](https://developer.apple.com/documentation/metal/mtldevice/supportsprimitivemotionblur) — A Boolean value that indicates whether the GPU device supports motion blur for ray tracing.
- [supportsRaytracingFromRender](https://developer.apple.com/documentation/metal/mtldevice/supportsraytracingfromrender) — A Boolean value that indicates whether you can call ray-tracing functions from a vertex or fragment shader.
- [supports32BitMSAA](https://developer.apple.com/documentation/metal/mtldevice/supports32bitmsaa) — A Boolean value that indicates whether the GPU can allocate 32-bit integer texture formats and resolve to 32-bit floating-point texture formats.
- [supportsPullModelInterpolation](https://developer.apple.com/documentation/metal/mtldevice/supportspullmodelinterpolation) — A Boolean value that indicates whether the GPU can compute multiple interpolations of a fragment function’s input.
- [supportsShaderBarycentricCoordinates](https://developer.apple.com/documentation/metal/mtldevice/supportsshaderbarycentriccoordinates) — A Boolean value that indicates whether the GPU supports barycentric coordinates.
- [areProgrammableSamplePositionsSupported](https://developer.apple.com/documentation/metal/mtldevice/areprogrammablesamplepositionssupported) — A Boolean value that indicates whether the GPU supports programmable sample positions.
- [areRasterOrderGroupsSupported](https://developer.apple.com/documentation/metal/mtldevice/arerasterordergroupssupported) — A Boolean value that indicates whether the GPU supports raster order groups.
- [areBarycentricCoordsSupported](https://developer.apple.com/documentation/metal/mtldevice/arebarycentriccoordssupported) — A Boolean value that indicates whether the GPU supports barycentric coordinates.
