# vertexStride

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/vertexstride>

Sets the stride, in bytes, between vertices in all the vertex buffer.

## Declaration

```swift
var vertexStride: Int { get set }
```

## Discussion

All keyframes share the same vertex stride. This stride needs to be a multiple of the size of the vertex format you provide in the [vertexFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/vertexformat) property.

Similarly, you are responsible for ensuring this stride matches the vertex format data type’s alignment.

Defaults to `0`, which signals the stride matches the size of the [vertexFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/vertexformat) data.
