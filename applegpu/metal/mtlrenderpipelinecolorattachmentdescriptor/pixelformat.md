# pixelFormat

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/pixelformat>

The pixel format of the color attachment’s texture.

## Declaration

```swift
var pixelFormat: MTLPixelFormat { get set }
```

## Discussion

The pixel format of the rendering pipeline state needs to be set to match the pixel format of the texture used by the selected color attachment; otherwise, an error occurs.

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Configuring render pipeline states
- [writeMask](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/writemask) — A bitmask that restricts which color channels are written into the texture.
- [MTLColorWriteMask](https://developer.apple.com/documentation/metal/mtlcolorwritemask) — Values used to specify a mask to permit or restrict writing to color channels of a color value.
