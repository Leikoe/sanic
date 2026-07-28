# iosurfacePlane

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.11, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltexture/iosurfaceplane>

The number of a plane within the underlying surface instance for the texture, if applicable.

## Declaration

```swift
var iosurfacePlane: Int { get }
```

## Discussion

The plane number applies to the [iosurfacePlane](https://developer.apple.com/documentation/metal/mtltexture/iosurfaceplane) property when it isn’t `nil`. The property’s value defaults to `0` for textures that don’t come from an [IOSurface](https://developer.apple.com/documentation/IOSurface/IOSurface) instance.

## See also

### Getting information about the IOSurface the texture was created from
- [iosurface](https://developer.apple.com/documentation/metal/mtltexture/iosurface) — A reference to the underlying surface instance for the texture, if applicable.
