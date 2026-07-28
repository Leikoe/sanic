# radiusBuffers

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/radiusbuffers>

Assigns a reference to a buffer containing, in turn, references to curve radii buffers.

## Declaration

```swift
var radiusBuffers: MTL4BufferRange { get set }
```

## Discussion

This property references a buffer that conceptually represents an array with one entry for each keyframe in the motion animation. Each one of these entries consists of a [MTL4BufferRange](https://developer.apple.com/documentation/metal/mtl4bufferrange) that, in turn, references a buffer containing the radii corresponding to the keyframe.

Metal interpolates curve radii according to the basis function you specify via [curveBasis](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/curvebasis).

You are responsible for ensuring the type of each radius matches the type property [radiusFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/radiusformat) specifies, that each radius is at least zero, and that the buffer address of the top-level buffer, as well as of buffer it references, is not zero.
