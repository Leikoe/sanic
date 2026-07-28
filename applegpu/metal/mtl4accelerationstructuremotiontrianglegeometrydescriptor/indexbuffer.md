# indexBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/indexbuffer>

Assigns an optional index buffer containing references to vertices in the vertex buffers you reference through the vertex buffers property.

## Declaration

```swift
var indexBuffer: MTL4BufferRange { get set }
```

## Discussion

You can set this property to `0`, the default, to avoid specifying an index buffer. All keyframes share the same index buffer.
