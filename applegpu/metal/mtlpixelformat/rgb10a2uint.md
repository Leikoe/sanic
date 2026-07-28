# MTLPixelFormat.rgb10a2Uint

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpixelformat/rgb10a2uint>

A 32-bit packed pixel format with four unsigned integer components: 10-bit red, 10-bit green, 10-bit blue, and 2-bit alpha.

## Declaration

```swift
case rgb10a2Uint
```

## Discussion

Pixel data is stored in red, green, blue, and alpha order, from least significant bit to most significant bit.

![image](https://docs-assets.developer.apple.com/published/1480c4a8ab5793367e5dc130522ae2d1/rgb10a2-layout-1%402x.png)

## See also

### Packed 32-bit pixel formats
- [MTLPixelFormat.bgr10a2Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bgr10a2unorm) — A 32-bit packed pixel format with four normalized unsigned integer components: 10-bit blue, 10-bit green, 10-bit red, and 2-bit alpha.
- [MTLPixelFormat.rgb10a2Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgb10a2unorm) — A 32-bit packed pixel format with four normalized unsigned integer components: 10-bit red, 10-bit green, 10-bit blue, and 2-bit alpha.
- [MTLPixelFormat.rg11b10Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rg11b10float) — 32-bit format with floating-point color components, 11 bits each for red and green and 10 bits for blue.
- [MTLPixelFormat.rgb9e5Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rgb9e5float) — Packed 32-bit format with floating-point color components: 9 bits each for RGB and 5 bits for an exponent shared by RGB, packed into 32 bits.
