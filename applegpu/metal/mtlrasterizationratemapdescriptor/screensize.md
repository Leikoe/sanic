# screenSize

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/screensize>

The size of the viewport coordinate system, in logical pixels.

## Declaration

```swift
var screenSize: MTLSize { get set }
```

## Discussion

Metal ignores the depth component of this property.

The viewport coordinate system’s origin is always at `(0,0)` and this property determines its size.
