# vertexFormat

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/vertexformat>

Defines the format of the vertices in the vertex buffers.

## Declaration

```swift
var vertexFormat: MTLAttributeFormat { get set }
```

## Discussion

All keyframes share the same vertex format. Defaults to `MTLAttributeFormatFloat3`, corresponding to three packed floating point numbers.
