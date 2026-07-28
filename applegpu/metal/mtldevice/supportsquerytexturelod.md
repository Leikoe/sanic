# supportsQueryTextureLOD

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportsquerytexturelod>

A Boolean value that indicates whether you can query the texture level of detail from within a shader.

## Declaration

```swift
var supportsQueryTextureLOD: Bool { get }
```

## See also

### Checking texture and sampler support
- [supports32BitFloatFiltering](https://developer.apple.com/documentation/metal/mtldevice/supports32bitfloatfiltering) — A Boolean value that indicates whether the GPU can filter a texture with a 32-bit floating-point format.
- [supportsBCTextureCompression](https://developer.apple.com/documentation/metal/mtldevice/supportsbctexturecompression) — A Boolean value that indicates whether you can use textures that use BC compression.
- [isDepth24Stencil8PixelFormatSupported](https://developer.apple.com/documentation/metal/mtldevice/isdepth24stencil8pixelformatsupported) — A Boolean value that indicates whether a device supports a packed depth-and-stencil pixel format.
- [readWriteTextureSupport](https://developer.apple.com/documentation/metal/mtldevice/readwritetexturesupport) — The GPU device’s texture support tier.
