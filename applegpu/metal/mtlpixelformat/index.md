# MTLPixelFormat

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpixelformat>

The data formats that describe the organization and characteristics of individual pixels in a texture.

## Declaration

```swift
enum MTLPixelFormat
```

## Overview

There are three varieties of pixel formats: ordinary, packed, and compressed. For ordinary and packed formats, the name of the pixel format specifies the order of components (such as `R`, `RG`, `RGB`, `RGBA`, `BGRA`), bits per component (such as `8`, `16`, `32`), and data type for the component (such as `Float`, `Sint`, `Snorm`, `Uint`, `Unorm`). If the pixel format name has the `_sRGB` suffix, then reading and writing pixel data applies sRGB gamma compression and decompression. The alpha component of sRGB pixel formats is always treated as a linear value. For compressed formats, the name of the pixel format specifies a compression family (such as `ASTC`, `BC`, `EAC`, `ETC2`, `PVRTC`).

> **Note:**
>  Pixel format availability and capabilities vary by feature set. See [Pixel Format Capabilities](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for more information.

### Storage characteristics

The number and size of each pixel component determines the storage size of each pixel format. For example, the storage size of [MTLPixelFormat.bgra8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bgra8unorm) is 32 bits (four 8-bit components) and the storage size of [MTLPixelFormat.bgr5A1Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bgr5a1unorm) is 16 bits (three 5-bit components and one 1-bit component).

For normalized signed integer formats (`Snorm`), values in the range `[-1.0, 1.0]` map to `[MIN_INT, MAX_INT]`, where `MIN_INT` is the most negative integer and `MAX_INT` is the most positive integer for the number of bits in the storage size. Positive values and zero distribute uniformly in the range `[0.0, 1.0]`, and negative integer values greater than `(MIN_INT + 1)` distribute uniformly in the range `(-1.0, 0.0)`.

> **Important:**
>  For `Snorm` formats, the values `MIN_INT` and `(MIN_INT + 1)` both map to `-1.0`.

For normalized unsigned integer formats (`Unorm`), values in the range `[0.0, 1.0]` are uniformly mapped to `[0, MAX_UINT]`, where `MAX_UINT` is the largest unsigned integer for the number of bits in the storage size.

Metal stores format data in little-endian byte order, with the least-significant byte at the lowest memory address. For formats with components that are themselves byte-aligned and more than one byte, Metal also stores each component in little-endian byte order.

See Table 7.7 in the [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf) (PDF) for details on pixel format normalization.

## Topics

### Ordinary 8-bit pixel formats
- [MTLPixelFormat.a8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/a8unorm) — Ordinary format with one 8-bit normalized unsigned integer component.
- [MTLPixelFormat.r8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/r8unorm) — Ordinary format with one 8-bit normalized unsigned integer component.
- [MTLPixelFormat.r8Unorm_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/r8unorm_srgb) — Ordinary format with one 8-bit normalized unsigned integer component with conversion between sRGB and linear space.
- [MTLPixelFormat.r8Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/r8snorm) — Ordinary format with one 8-bit normalized signed integer component.
- [MTLPixelFormat.r8Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/r8uint) — Ordinary format with one 8-bit unsigned integer component.
- [MTLPixelFormat.r8Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/r8sint) — Ordinary format with one 8-bit signed integer component.

### Ordinary 16-bit pixel formats
- [MTLPixelFormat.r16Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/r16unorm) — Ordinary format with one 16-bit normalized unsigned integer component.
- [MTLPixelFormat.r16Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/r16snorm) — Ordinary format with one 16-bit normalized signed integer component.
- [MTLPixelFormat.r16Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/r16uint) — Ordinary format with one 16-bit unsigned integer component.
- [MTLPixelFormat.r16Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/r16sint) — Ordinary format with one 16-bit signed integer component.
- [MTLPixelFormat.r16Float](https://developer.apple.com/documentation/metal/mtlpixelformat/r16float) — Ordinary format with one 16-bit floating-point component.
- [MTLPixelFormat.rg8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rg8unorm) — Ordinary format with two 8-bit normalized unsigned integer components.
- [MTLPixelFormat.rg8Unorm_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/rg8unorm_srgb) — Ordinary format with two 8-bit normalized unsigned integer components with conversion between sRGB and linear space.
- [MTLPixelFormat.rg8Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rg8snorm) — Ordinary format with two 8-bit normalized signed integer components.
- [MTLPixelFormat.rg8Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rg8uint) — Ordinary format with two 8-bit unsigned integer components.
- [MTLPixelFormat.rg8Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rg8sint) — Ordinary format with two 8-bit signed integer components.

### Packed 16-bit pixel formats
- [MTLPixelFormat.b5g6r5Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/b5g6r5unorm) — Packed 16-bit format with normalized unsigned integer color components: 5 bits for blue, 6 bits for green, 5 bits for red, packed into 16 bits.
- [MTLPixelFormat.a1bgr5Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/a1bgr5unorm) — Packed 16-bit format with normalized unsigned integer color components: 5 bits each for BGR and 1 for alpha, packed into 16 bits.
- [MTLPixelFormat.abgr4Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/abgr4unorm) — Packed 16-bit format with normalized unsigned integer color components: 4 bits each for ABGR, packed into 16 bits.
- [MTLPixelFormat.bgr5A1Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bgr5a1unorm) — Packed 16-bit format with normalized unsigned integer color components: 5 bits each for BGR and 1 for alpha, packed into 16 bits.

### Ordinary 32-bit pixel formats
- [MTLPixelFormat.r32Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/r32uint) — Ordinary format with one 32-bit unsigned integer component.
- [MTLPixelFormat.r32Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/r32sint) — Ordinary format with one 32-bit signed integer component.
- [MTLPixelFormat.r32Float](https://developer.apple.com/documentation/metal/mtlpixelformat/r32float) — Ordinary format with one 32-bit floating-point component.
- [MTLPixelFormat.rg16Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rg16unorm) — Ordinary format with two 16-bit normalized unsigned integer components.
- [MTLPixelFormat.rg16Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rg16snorm) — Ordinary format with two 16-bit normalized signed integer components.
- [MTLPixelFormat.rg16Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rg16uint) — Ordinary format with two 16-bit unsigned integer components.
- [MTLPixelFormat.rg16Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rg16sint) — Ordinary format with two 16-bit signed integer components.
- [MTLPixelFormat.rg16Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rg16float) — Ordinary format with two 16-bit floating-point components.
- [MTLPixelFormat.rgba8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8unorm) — Ordinary format with four 8-bit normalized unsigned integer components in RGBA order.
- [MTLPixelFormat.rgba8Unorm_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8unorm_srgb) — Ordinary format with four 8-bit normalized unsigned integer components in RGBA order with conversion between sRGB and linear space.
- [MTLPixelFormat.rgba8Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8snorm) — Ordinary format with four 8-bit normalized signed integer components in RGBA order.
- [MTLPixelFormat.rgba8Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8uint) — Ordinary format with four 8-bit unsigned integer components in RGBA order.
- [MTLPixelFormat.rgba8Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8sint) — Ordinary format with four 8-bit signed integer components in RGBA order.
- [MTLPixelFormat.bgra8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bgra8unorm) — Ordinary format with four 8-bit normalized unsigned integer components in BGRA order.
- [MTLPixelFormat.bgra8Unorm_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/bgra8unorm_srgb) — Ordinary format with four 8-bit normalized unsigned integer components in BGRA order with conversion between sRGB and linear space.

### Packed 32-bit pixel formats
- [MTLPixelFormat.bgr10a2Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bgr10a2unorm) — A 32-bit packed pixel format with four normalized unsigned integer components: 10-bit blue, 10-bit green, 10-bit red, and 2-bit alpha.
- [MTLPixelFormat.rgb10a2Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgb10a2unorm) — A 32-bit packed pixel format with four normalized unsigned integer components: 10-bit red, 10-bit green, 10-bit blue, and 2-bit alpha.
- [MTLPixelFormat.rgb10a2Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgb10a2uint) — A 32-bit packed pixel format with four unsigned integer components: 10-bit red, 10-bit green, 10-bit blue, and 2-bit alpha.
- [MTLPixelFormat.rg11b10Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rg11b10float) — 32-bit format with floating-point color components, 11 bits each for red and green and 10 bits for blue.
- [MTLPixelFormat.rgb9e5Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rgb9e5float) — Packed 32-bit format with floating-point color components: 9 bits each for RGB and 5 bits for an exponent shared by RGB, packed into 32 bits.

### Ordinary 64-bit pixel formats
- [MTLPixelFormat.rg32Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rg32uint) — Ordinary format with two 32-bit unsigned integer components.
- [MTLPixelFormat.rg32Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rg32sint) — Ordinary format with two 32-bit signed integer components.
- [MTLPixelFormat.rg32Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rg32float) — Ordinary format with two 32-bit floating-point components.
- [MTLPixelFormat.rgba16Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba16unorm) — Ordinary format with four 16-bit normalized unsigned integer components in RGBA order.
- [MTLPixelFormat.rgba16Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba16snorm) — Ordinary format with four 16-bit normalized signed integer components in RGBA order.
- [MTLPixelFormat.rgba16Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba16uint) — Ordinary format with four 16-bit unsigned integer components in RGBA order.
- [MTLPixelFormat.rgba16Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba16sint) — Ordinary format with four 16-bit signed integer components in RGBA order.
- [MTLPixelFormat.rgba16Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba16float) — Ordinary format with four 16-bit floating-point components in RGBA order.

### Ordinary 128-bit pixel formats
- [MTLPixelFormat.rgba32Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba32uint) — Ordinary format with four 32-bit unsigned integer components in RGBA order.
- [MTLPixelFormat.rgba32Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba32sint) — Ordinary format with four 32-bit signed integer components in RGBA order.
- [MTLPixelFormat.rgba32Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba32float) — Ordinary format with four 32-bit floating-point components in RGBA order.

### Compressed PVRTC pixel formats
- [MTLPixelFormat.pvrtc_rgb_2bpp](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgb_2bpp) — A compressed format that uses PVRTC compression and 2bpp for RGB components.
- [MTLPixelFormat.pvrtc_rgb_2bpp_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgb_2bpp_srgb) — A compressed format that uses PVRTC compression and 2bpp for RGB components with a conversion between sRGB and linear space.
- [MTLPixelFormat.pvrtc_rgb_4bpp](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgb_4bpp) — A compressed format that uses PVRTC compression and 4bpp for RGB components.
- [MTLPixelFormat.pvrtc_rgb_4bpp_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgb_4bpp_srgb) — A compressed format that uses PVRTC compression and 4bpp for RGB components with a conversion between sRGB and linear space.
- [MTLPixelFormat.pvrtc_rgba_2bpp](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgba_2bpp) — A compressed format that uses PVRTC compression and 2bpp for RGBA components.
- [MTLPixelFormat.pvrtc_rgba_2bpp_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgba_2bpp_srgb) — A compressed format that uses PVRTC compression and 2bpp for RGBA components with a conversion between sRGB and linear space.
- [MTLPixelFormat.pvrtc_rgba_4bpp](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgba_4bpp) — A compressed format that uses PVRTC compression and 4bpp for RGBA components.
- [MTLPixelFormat.pvrtc_rgba_4bpp_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/pvrtc_rgba_4bpp_srgb) — A compressed format that uses PVRTC compression and 4bpp for RGBA components with a conversion between sRGB and linear space.

### Compressed EAC/ETC pixel formats
- [MTLPixelFormat.eac_r11Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_r11unorm) — Compressed format using EAC compression with one normalized unsigned integer component.
- [MTLPixelFormat.eac_r11Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_r11snorm) — Compressed format using EAC compression with one normalized signed integer component.
- [MTLPixelFormat.eac_rg11Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_rg11unorm) — Compressed format using EAC compression with two normalized unsigned integer components.
- [MTLPixelFormat.eac_rg11Snorm](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_rg11snorm) — Compressed format using EAC compression with two normalized signed integer components.
- [MTLPixelFormat.eac_rgba8](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_rgba8) — Compressed format using EAC compression with four 8-bit components.
- [MTLPixelFormat.eac_rgba8_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/eac_rgba8_srgb) — Compressed format using EAC compression with four 8-bit components with conversion between sRGB and linear space.
- [MTLPixelFormat.etc2_rgb8](https://developer.apple.com/documentation/metal/mtlpixelformat/etc2_rgb8) — Compressed format using ETC2 compression with three 8-bit components.
- [MTLPixelFormat.etc2_rgb8_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/etc2_rgb8_srgb) — Compressed format using ETC2 compression with three 8-bit components with conversion between sRGB and linear space.
- [MTLPixelFormat.etc2_rgb8a1](https://developer.apple.com/documentation/metal/mtlpixelformat/etc2_rgb8a1) — Compressed format using ETC2 compression with four 8-bit components.
- [MTLPixelFormat.etc2_rgb8a1_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/etc2_rgb8a1_srgb) — Compressed format using ETC2 compression with four 8-bit components with conversion between sRGB and linear space.

### Compressed ASTC pixel formats
- [MTLPixelFormat.astc_4x4_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_4x4_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 4, and a block height of 4.
- [MTLPixelFormat.astc_5x4_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_5x4_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 5, and a block height of 4.
- [MTLPixelFormat.astc_5x5_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_5x5_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 5, and a block height of 5.
- [MTLPixelFormat.astc_6x5_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_6x5_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 6, and a block height of 5.
- [MTLPixelFormat.astc_6x6_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_6x6_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 6, and a block height of 6.
- [MTLPixelFormat.astc_8x5_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x5_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 8, and a block height of 5.
- [MTLPixelFormat.astc_8x6_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x6_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 8, and a block height of 6.
- [MTLPixelFormat.astc_8x8_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x8_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 8, and a block height of 8.
- [MTLPixelFormat.astc_10x5_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x5_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 10, and a block height of 5.
- [MTLPixelFormat.astc_10x6_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x6_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 10, and a block height of 6.
- [MTLPixelFormat.astc_10x8_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x8_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 10, and a block height of 8.
- [MTLPixelFormat.astc_10x10_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x10_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 10, and a block height of 10.
- [MTLPixelFormat.astc_12x10_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_12x10_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 12, and a block height of 10.
- [MTLPixelFormat.astc_12x12_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_12x12_srgb) — ASTC-compressed format with low-dynamic-range content, conversion between sRGB and linear space, a block width of 12, and a block height of 12.
- [MTLPixelFormat.astc_4x4_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_4x4_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 4, and a block height of 4.
- [MTLPixelFormat.astc_5x4_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_5x4_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 5, and a block height of 4.
- [MTLPixelFormat.astc_5x5_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_5x5_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 5, and a block height of 5.
- [MTLPixelFormat.astc_6x5_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_6x5_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 6, and a block height of 5.
- [MTLPixelFormat.astc_6x6_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_6x6_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 6, and a block height of 6.
- [MTLPixelFormat.astc_8x5_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x5_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 8, and a block height of 5.
- [MTLPixelFormat.astc_8x6_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x6_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 8, and a block height of 6.
- [MTLPixelFormat.astc_8x8_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x8_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 8, and a block height of 8.
- [MTLPixelFormat.astc_10x5_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x5_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 10, and a block height of 5.
- [MTLPixelFormat.astc_10x6_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x6_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 10, and a block height of 6.
- [MTLPixelFormat.astc_10x8_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x8_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 10, and a block height of 8.
- [MTLPixelFormat.astc_10x10_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x10_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 10, and a block height of 10.
- [MTLPixelFormat.astc_12x10_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_12x10_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 12, and a block height of 10.
- [MTLPixelFormat.astc_12x12_ldr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_12x12_ldr) — ASTC-compressed format with low-dynamic-range content, a block width of 12, and a block height of 12.
- [MTLPixelFormat.astc_4x4_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_4x4_hdr) — ASTC-compressed format with high-dynamic-range content, a block width of 4, and a block height of 4.
- [MTLPixelFormat.astc_5x4_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_5x4_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 5, and a block height of 4.
- [MTLPixelFormat.astc_5x5_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_5x5_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 5, and a block height of 5.
- [MTLPixelFormat.astc_6x5_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_6x5_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 6, and a block height of 5.
- [MTLPixelFormat.astc_6x6_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_6x6_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 6, and a block height of 6.
- [MTLPixelFormat.astc_8x5_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x5_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 8, and a block height of 5.
- [MTLPixelFormat.astc_8x6_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x6_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 8, and a block height of 6.
- [MTLPixelFormat.astc_8x8_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_8x8_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 8, and a block height of 8.
- [MTLPixelFormat.astc_10x5_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x5_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 10, and a block height of 5.
- [MTLPixelFormat.astc_10x6_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x6_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 10, and a block height of 6.
- [MTLPixelFormat.astc_10x8_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x8_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 10, and a block height of 8.
- [MTLPixelFormat.astc_10x10_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_10x10_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 10, and a block height of 10.
- [MTLPixelFormat.astc_12x10_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_12x10_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 12, and a block height of 10.
- [MTLPixelFormat.astc_12x12_hdr](https://developer.apple.com/documentation/metal/mtlpixelformat/astc_12x12_hdr) — ASTC-compressed format with high-dynamic range content, a block width of 12, and a block height of 12.

### Compressed BC pixel formats
- [MTLPixelFormat.bc1_rgba](https://developer.apple.com/documentation/metal/mtlpixelformat/bc1_rgba) — Compressed format with two 16-bit color components and one 32-bit descriptor component.
- [MTLPixelFormat.bc1_rgba_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/bc1_rgba_srgb) — Compressed format with two 16-bit color components and one 32-bit descriptor component, with conversion between sRGB and linear space.
- [MTLPixelFormat.bc2_rgba](https://developer.apple.com/documentation/metal/mtlpixelformat/bc2_rgba) — Compressed format with two 64-bit chunks. The first chunk contains two 8-bit alpha components and one 48-bit descriptor component. The second chunk contains two 16-bit color components and one 32-bit descriptor component.
- [MTLPixelFormat.bc2_rgba_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/bc2_rgba_srgb) — Compressed format with two 64-bit chunks, with conversion between sRGB and linear space. The first chunk contains two 8-bit alpha components and one 48-bit descriptor component. The second chunk contains two 16-bit color components and one 32-bit descriptor component.
- [MTLPixelFormat.bc3_rgba](https://developer.apple.com/documentation/metal/mtlpixelformat/bc3_rgba) — Compressed format with two 64-bit chunks. The first chunk contains two 8-bit alpha components and one 48-bit descriptor component. The second chunk contains two 16-bit color components and one 32-bit descriptor component.
- [MTLPixelFormat.bc3_rgba_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/bc3_rgba_srgb) — Compressed format with two 64-bit chunks, with conversion between sRGB and linear space. The first chunk contains two 8-bit alpha components and one 48-bit descriptor component. The second chunk contains two 16-bit color components and one 32-bit descriptor component.
- [MTLPixelFormat.bc4_rUnorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bc4_runorm) — Compressed format with one normalized unsigned integer component.
- [MTLPixelFormat.bc4_rSnorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bc4_rsnorm) — Compressed format with one normalized signed integer component.
- [MTLPixelFormat.bc5_rgUnorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bc5_rgunorm) — Compressed format with two normalized unsigned integer components.
- [MTLPixelFormat.bc5_rgSnorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bc5_rgsnorm) — Compressed format with two normalized signed integer components.
- [MTLPixelFormat.bc6H_rgbFloat](https://developer.apple.com/documentation/metal/mtlpixelformat/bc6h_rgbfloat) — Compressed format with four floating-point components.
- [MTLPixelFormat.bc6H_rgbuFloat](https://developer.apple.com/documentation/metal/mtlpixelformat/bc6h_rgbufloat) — Compressed format with four unsigned floating-point components.
- [MTLPixelFormat.bc7_rgbaUnorm](https://developer.apple.com/documentation/metal/mtlpixelformat/bc7_rgbaunorm) — Compressed format with four normalized unsigned integer components.
- [MTLPixelFormat.bc7_rgbaUnorm_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/bc7_rgbaunorm_srgb) — Compressed format with four normalized unsigned integer components, with conversion between sRGB and linear space.

### YUV pixel formats
- [MTLPixelFormat.gbgr422](https://developer.apple.com/documentation/metal/mtlpixelformat/gbgr422) — A pixel format where the red and green components are subsampled horizontally.
- [MTLPixelFormat.bgrg422](https://developer.apple.com/documentation/metal/mtlpixelformat/bgrg422) — A pixel format where the red and green components are subsampled horizontally.

### Depth and stencil pixel formats
- [MTLPixelFormat.depth16Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/depth16unorm) — A pixel format for a depth-render target that has a 16-bit normalized, unsigned-integer component.
- [MTLPixelFormat.depth32Float](https://developer.apple.com/documentation/metal/mtlpixelformat/depth32float) — A pixel format with one 32-bit floating-point component, used for a depth render target.
- [MTLPixelFormat.stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/stencil8) — A pixel format with an 8-bit unsigned integer component, used for a stencil render target.
- [MTLPixelFormat.depth24Unorm_stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/depth24unorm_stencil8) — A 32-bit combined depth and stencil pixel format with a 24-bit normalized unsigned integer for depth and an 8-bit unsigned integer for stencil.
- [MTLPixelFormat.depth32Float_stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/depth32float_stencil8) — A 40-bit combined depth and stencil pixel format with a 32-bit floating-point value for depth and an 8-bit unsigned integer for stencil.
- [MTLPixelFormat.x32_stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/x32_stencil8) — A stencil pixel format used to read the stencil value from a texture with a combined 32-bit depth and 8-bit stencil value.
- [MTLPixelFormat.x24_stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/x24_stencil8) — A stencil pixel format used to read the stencil value from a texture with a combined 24-bit depth and 8-bit stencil value.

### Extended range and wide color pixel formats
- [MTLPixelFormat.bgra10_xr](https://developer.apple.com/documentation/metal/mtlpixelformat/bgra10_xr) — A 64-bit extended-range pixel format with four fixed-point components of 10-bit blue, 10-bit green, 10-bit red, and 10-bit alpha.
- [MTLPixelFormat.bgra10_xr_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/bgra10_xr_srgb) — A 64-bit extended-range pixel format with sRGB conversion and four fixed-point components of 10-bit blue, 10-bit green, 10-bit red, and 10-bit alpha.
- [MTLPixelFormat.bgr10_xr](https://developer.apple.com/documentation/metal/mtlpixelformat/bgr10_xr) — A 32-bit extended-range pixel format with three fixed-point components of 10-bit blue, 10-bit green, and 10-bit red.
- [MTLPixelFormat.bgr10_xr_srgb](https://developer.apple.com/documentation/metal/mtlpixelformat/bgr10_xr_srgb) — A 32-bit extended-range pixel format with sRGB conversion and three fixed-point components of 10-bit blue, 10-bit green, and 10-bit red.

### Sentinel values
- [MTLPixelFormat.invalid](https://developer.apple.com/documentation/metal/mtlpixelformat/invalid) — The default value of the pixel format for the `MTLRenderPipelineState`. You cannot create a texture with this value.

### Enumeration Cases
- [MTLPixelFormat.unspecialized](https://developer.apple.com/documentation/metal/mtlpixelformat/unspecialized)

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlpixelformat/init(rawvalue:))

## See also

### Texture basics
- [Understanding color-renderable pixel format sizes](https://developer.apple.com/documentation/metal/understanding-color-renderable-pixel-format-sizes) — Know the size limits of color render targets in Apple GPUs based on the target’s pixel format.
- [Optimizing texture data](https://developer.apple.com/documentation/metal/optimizing-texture-data) — Optimize a texture’s data to improve GPU or CPU access.
- [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) — A resource that holds formatted image data.
- [MTLTextureCompressionType](https://developer.apple.com/documentation/metal/mtltexturecompressiontype)
- [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) — An instance that you use to configure new Metal texture instances.
- [MTKTextureLoader](https://developer.apple.com/documentation/MetalKit/MTKTextureLoader) — An object that creates textures from existing data in common image formats.
- [MTLSharedTextureHandle](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle) — A texture handle that can be shared across process address space boundaries.
