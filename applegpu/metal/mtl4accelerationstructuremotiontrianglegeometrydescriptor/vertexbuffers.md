# vertexBuffers

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/vertexbuffers>

Assigns a buffer where each entry contains a reference to a vertex buffer.

## Declaration

```swift
var vertexBuffers: MTL4BufferRange { get set }
```

## Discussion

This property references a buffer that conceptually represents an array with one entry for each keyframe in the motion animation. Each one of these entries consists of a [MTL4BufferRange](https://developer.apple.com/documentation/metal/mtl4bufferrange) that, in turn, references a vertex buffer containing the vertex data for the keyframe.

You are responsible for ensuring the buffer address is not zero for the top-level buffer, as well as for all the vertex buffers it references.
