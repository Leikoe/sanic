# vertexBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexbuffer>

Associates a vertex buffer containing triangle vertices.

## Declaration

```swift
var vertexBuffer: MTL4BufferRange { get set }
```

## Discussion

You are responsible for ensuring that the format of all vertex positions match the [vertexFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexformat) property, and that the buffer address for the buffer range is not zero.
