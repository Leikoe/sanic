# MTLPixelFormat.depth32Float_stencil8

*Case · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpixelformat/depth32float_stencil8>

A 40-bit combined depth and stencil pixel format with a 32-bit floating-point value for depth and an 8-bit unsigned integer for stencil.

## Declaration

```swift
case depth32Float_stencil8
```

## Discussion

When using this format, some Metal device objects allocate 64-bits per pixel.

## See also

### Depth and stencil pixel formats
- [MTLPixelFormat.depth16Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/depth16unorm) — A pixel format for a depth-render target that has a 16-bit normalized, unsigned-integer component.
- [MTLPixelFormat.depth32Float](https://developer.apple.com/documentation/metal/mtlpixelformat/depth32float) — A pixel format with one 32-bit floating-point component, used for a depth render target.
- [MTLPixelFormat.stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/stencil8) — A pixel format with an 8-bit unsigned integer component, used for a stencil render target.
- [MTLPixelFormat.depth24Unorm_stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/depth24unorm_stencil8) — A 32-bit combined depth and stencil pixel format with a 24-bit normalized unsigned integer for depth and an 8-bit unsigned integer for stencil.
- [MTLPixelFormat.x32_stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/x32_stencil8) — A stencil pixel format used to read the stencil value from a texture with a combined 32-bit depth and 8-bit stencil value.
- [MTLPixelFormat.x24_stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/x24_stencil8) — A stencil pixel format used to read the stencil value from a texture with a combined 24-bit depth and 8-bit stencil value.
