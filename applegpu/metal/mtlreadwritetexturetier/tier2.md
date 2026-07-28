# MTLReadWriteTextureTier.tier2

*Case · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlreadwritetexturetier/tier2>

Indicates the system supports tier 2 read-write textures.

## Declaration

```swift
case tier2
```

## Discussion

Tier 2 read-write textures support the following pixel formats (in addition to [MTLReadWriteTextureTier.tier1](https://developer.apple.com/documentation/metal/mtlreadwritetexturetier/tier1)):

- [MTLPixelFormat.rgba32Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba32float)

- [MTLPixelFormat.rgba32Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba32uint)

- [MTLPixelFormat.rgba32Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba32sint)

- [MTLPixelFormat.rgba16Float](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba16float)

- [MTLPixelFormat.rgba16Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba16uint)

- [MTLPixelFormat.rgba16Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba16sint)

- [MTLPixelFormat.rgba8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8unorm)

- [MTLPixelFormat.rgba8Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8uint)

- [MTLPixelFormat.rgba8Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/rgba8sint)

- [MTLPixelFormat.r16Float](https://developer.apple.com/documentation/metal/mtlpixelformat/r16float)

- [MTLPixelFormat.r16Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/r16uint)

- [MTLPixelFormat.r16Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/r16sint)

- [MTLPixelFormat.r8Unorm](https://developer.apple.com/documentation/metal/mtlpixelformat/r8unorm)

- [MTLPixelFormat.r8Uint](https://developer.apple.com/documentation/metal/mtlpixelformat/r8uint)

- [MTLPixelFormat.r8Sint](https://developer.apple.com/documentation/metal/mtlpixelformat/r8sint)

## See also

### Enumeration cases
- [MTLReadWriteTextureTier.tier1](https://developer.apple.com/documentation/metal/mtlreadwritetexturetier/tier1) — Indicates the system supports tier 1 read-write textures.
- [MTLReadWriteTextureTier.tierNone](https://developer.apple.com/documentation/metal/mtlreadwritetexturetier/tiernone) — Indicates the system doesn’t support read-write textures.
