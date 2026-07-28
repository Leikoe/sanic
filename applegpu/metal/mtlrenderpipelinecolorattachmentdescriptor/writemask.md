# writeMask

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/writemask>

A bitmask that restricts which color channels are written into the texture.

## Declaration

```swift
var writeMask: MTLColorWriteMask { get set }
```

## Discussion

The default value of `writeMask` is all ones, [all](https://developer.apple.com/documentation/metal/mtlcolorwritemask/all), which allows all color channels to be blended. The `MTLColorWriteMask` values `MTLColorWriteMaskRed`, `MTLColorWriteMaskGreen`, `MTLColorWriteMaskBlue`, and `MTLColorWriteMaskAlpha` limit blending to one color channel, and these values can be bitwise combined. `MTLColorWriteMaskNone` does not allow any color channels to be blended.

## See also

### Configuring render pipeline states
- [pixelFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/pixelformat) — The pixel format of the color attachment’s texture.
- [MTLColorWriteMask](https://developer.apple.com/documentation/metal/mtlcolorwritemask) — Values used to specify a mask to permit or restrict writing to color channels of a color value.
