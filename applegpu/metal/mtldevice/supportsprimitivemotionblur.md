# supportsPrimitiveMotionBlur

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportsprimitivemotionblur>

A Boolean value that indicates whether the GPU device supports motion blur for ray tracing.

## Declaration

```swift
var supportsPrimitiveMotionBlur: Bool { get }
```

## See also

### Checking render support
- [supportsRaytracing](https://developer.apple.com/documentation/metal/mtldevice/supportsraytracing) — A Boolean value that indicates whether the GPU device supports ray tracing.
- [supportsRaytracingFromRender](https://developer.apple.com/documentation/metal/mtldevice/supportsraytracingfromrender) — A Boolean value that indicates whether you can call ray-tracing functions from a vertex or fragment shader.
- [supports32BitMSAA](https://developer.apple.com/documentation/metal/mtldevice/supports32bitmsaa) — A Boolean value that indicates whether the GPU can allocate 32-bit integer texture formats and resolve to 32-bit floating-point texture formats.
- [supportsPullModelInterpolation](https://developer.apple.com/documentation/metal/mtldevice/supportspullmodelinterpolation) — A Boolean value that indicates whether the GPU can compute multiple interpolations of a fragment function’s input.
- [supportsShaderBarycentricCoordinates](https://developer.apple.com/documentation/metal/mtldevice/supportsshaderbarycentriccoordinates) — A Boolean value that indicates whether the GPU supports barycentric coordinates.
- [supportsVertexAmplificationCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportsvertexamplificationcount(_:)) — Returns a Boolean value that indicates whether the GPU supports an amplification factor.
- [areProgrammableSamplePositionsSupported](https://developer.apple.com/documentation/metal/mtldevice/areprogrammablesamplepositionssupported) — A Boolean value that indicates whether the GPU supports programmable sample positions.
- [areRasterOrderGroupsSupported](https://developer.apple.com/documentation/metal/mtldevice/arerasterordergroupssupported) — A Boolean value that indicates whether the GPU supports raster order groups.
- [areBarycentricCoordsSupported](https://developer.apple.com/documentation/metal/mtldevice/arebarycentriccoordssupported) — A Boolean value that indicates whether the GPU supports barycentric coordinates.
