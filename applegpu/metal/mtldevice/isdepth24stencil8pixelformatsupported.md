# isDepth24Stencil8PixelFormatSupported

*Instance Property · Mac Catalyst 13.0, macOS 10.11*

<https://developer.apple.com/documentation/metal/mtldevice/isdepth24stencil8pixelformatsupported>

A Boolean value that indicates whether a device supports a packed depth-and-stencil pixel format.

## Declaration

```swift
var isDepth24Stencil8PixelFormatSupported: Bool { get }
```

## Discussion

If the value is [true](https://developer.apple.com/documentation/Swift/true), the device supports the [MTLPixelFormat.depth24Unorm_stencil8](https://developer.apple.com/documentation/metal/mtlpixelformat/depth24unorm_stencil8) pixel format.

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Checking texture and sampler support
- [supports32BitFloatFiltering](https://developer.apple.com/documentation/metal/mtldevice/supports32bitfloatfiltering) — A Boolean value that indicates whether the GPU can filter a texture with a 32-bit floating-point format.
- [supportsBCTextureCompression](https://developer.apple.com/documentation/metal/mtldevice/supportsbctexturecompression) — A Boolean value that indicates whether you can use textures that use BC compression.
- [supportsQueryTextureLOD](https://developer.apple.com/documentation/metal/mtldevice/supportsquerytexturelod) — A Boolean value that indicates whether you can query the texture level of detail from within a shader.
- [readWriteTextureSupport](https://developer.apple.com/documentation/metal/mtldevice/readwritetexturesupport) — The GPU device’s texture support tier.
