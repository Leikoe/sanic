# triangleCount

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/trianglecount>

The number of triangles in the buffers.

## Declaration

```swift
var triangleCount: Int { get set }
```

## Discussion

If the triangle descriptor contains an index buffer, then the index buffer needs to provide indices for this many triangles. If the triangle descriptor doesn’t provide an index buffer, then the vertex buffer provides 3 vertices for each triangle.
