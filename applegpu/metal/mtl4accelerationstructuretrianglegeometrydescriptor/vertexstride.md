# vertexStride

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexstride>

Sets the stride, in bytes, between vertices in the vertex buffer.

## Declaration

```swift
var vertexStride: Int { get set }
```

## Discussion

The stride you specify needs to be a multiple of the size of the vertex format you provide in the [vertexFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexformat) property. Similarly, you are responsible for ensuring this stride matches the vertex format data type’s alignment.

Defaults to `0`, which signals the stride matches the size of the [vertexFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexformat) data.
