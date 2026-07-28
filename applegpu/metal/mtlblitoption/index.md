# MTLBlitOption

*Structure · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitoption>

The options that enable behavior for some blit operations.

## Declaration

```swift
struct MTLBlitOption
```

## Topics

### Depth and stencil buffer options
- [depthFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/depthfromdepthstencil) — A blit option that copies the depth portion of a combined depth and stencil texture to or from a buffer.
- [stencilFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/stencilfromdepthstencil) — A blit option that copies the stencil portion of a combined depth and stencil texture to or from a buffer.

### Texture compression options
- [rowLinearPVRTC](https://developer.apple.com/documentation/metal/mtlblitoption/rowlinearpvrtc) — A blit option that copies PVRTC data between a texture and a buffer.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlblitoption/init(rawvalue:)) — Creates a blit option from a raw value.

## See also

### Encoding a blit pass
- [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) — Encodes commands that copy and modify resources for a single blit pass.
