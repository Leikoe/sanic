# vertexFormat

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexformat>

Describes the format of the vertices in the vertex buffer.

## Declaration

```swift
var vertexFormat: MTLAttributeFormat { get set }
```

## Discussion

This property controls the format of the position attribute of the vertices the [vertexBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexbuffer) references.

The format defaults to `MTLAttributeFormatFloat3`, corresponding to three packed floating point numbers.
