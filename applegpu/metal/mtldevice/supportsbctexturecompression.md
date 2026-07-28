# supportsBCTextureCompression

*Instance Property · iOS 16.4, iPadOS 16.4, Mac Catalyst 16.4, macOS 11.0, tvOS 16.4, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportsbctexturecompression>

A Boolean value that indicates whether you can use textures that use BC compression.

## Declaration

```swift
var supportsBCTextureCompression: Bool { get }
```

## See also

### Checking texture and sampler support
- [supports32BitFloatFiltering](https://developer.apple.com/documentation/metal/mtldevice/supports32bitfloatfiltering) — A Boolean value that indicates whether the GPU can filter a texture with a 32-bit floating-point format.
- [isDepth24Stencil8PixelFormatSupported](https://developer.apple.com/documentation/metal/mtldevice/isdepth24stencil8pixelformatsupported) — A Boolean value that indicates whether a device supports a packed depth-and-stencil pixel format.
- [supportsQueryTextureLOD](https://developer.apple.com/documentation/metal/mtldevice/supportsquerytexturelod) — A Boolean value that indicates whether you can query the texture level of detail from within a shader.
- [readWriteTextureSupport](https://developer.apple.com/documentation/metal/mtldevice/readwritetexturesupport) — The GPU device’s texture support tier.
