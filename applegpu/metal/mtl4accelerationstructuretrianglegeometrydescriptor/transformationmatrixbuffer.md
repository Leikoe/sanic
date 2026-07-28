# transformationMatrixBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/transformationmatrixbuffer>

Assigns an optional reference to a buffer containing a `float4x3` transformation matrix.

## Declaration

```swift
var transformationMatrixBuffer: MTL4BufferRange { get set }
```

## Discussion

When the buffer address is non-zero, Metal applies this transform to the vertex data positions when building the acceleration structure.

Building an acceleration structure with a descriptor that specifies this property doesn’t modify the contents of the input `vertexBuffer`.
