# MTLPixelFormat.eac_r11Snorm

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 14.0, macOS 11.0, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpixelformat/eac_r11snorm>

Compressed format using EAC compression with one normalized signed integer component.

## Declaration

```swift
case eac_r11Snorm
```

## Discussion

Only [MTLTextureType.type2D](https://developer.apple.com/documentation/metal/mtltexturetype/type2d), [MTLTextureType.type2DArray](https://developer.apple.com/documentation/metal/mtltexturetype/type2darray), and [MTLTextureType.typeCube](https://developer.apple.com/documentation/metal/mtltexturetype/typecube) textures are supported.

## See also

### Compressed EAC/ETC pixel formats
- [MTLPixelFormat.eac_r11Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_r11unorm) — Compressed format using EAC compression with one normalized unsigned integer component.
- [MTLPixelFormat.eac_rg11Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_rg11unorm) — Compressed format using EAC compression with two normalized unsigned integer components.
- [MTLPixelFormat.eac_rg11Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_rg11snorm) — Compressed format using EAC compression with two normalized signed integer components.
- [MTLPixelFormat.eac_rgba8](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_rgba8) — Compressed format using EAC compression with four 8-bit components.
- [MTLPixelFormat.eac_rgba8_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_rgba8_srgb) — Compressed format using EAC compression with four 8-bit components with conversion between sRGB and linear space.
- [MTLPixelFormat.etc2_rgb8](https://developer.apple.com/documentation/metal/mtlpixelformat/etc2_rgb8) — Compressed format using ETC2 compression with three 8-bit components.
- [MTLPixelFormat.etc2_rgb8_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/etc2_rgb8_srgb) — Compressed format using ETC2 compression with three 8-bit components with conversion between sRGB and linear space.
- [MTLPixelFormat.etc2_rgb8a1](https://developer.apple.com/documentation/metal/mtlpixelformat/etc2_rgb8a1) — Compressed format using ETC2 compression with four 8-bit components.
- [MTLPixelFormat.etc2_rgb8a1_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/etc2_rgb8a1_srgb) — Compressed format using ETC2 compression with four 8-bit components with conversion between sRGB and linear space.
