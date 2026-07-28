# imageblockSampleLength

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/imageblocksamplelength>

The per-sample size, in bytes, of the largest explicit imageblock layout in the render pass.

## Declaration

```swift
var imageblockSampleLength: Int { get set }
```

## Discussion

If `imageBlockSampleLength` isn’t specified, Metal determines the imageblock sample length from the render pass attachment formats.  If any render pipelines bound to the encoder reference imageblocks with explicit layout, you need to set this property.

## See also

### Specifying tile shading parameters
- [threadgroupMemoryLength](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/threadgroupmemorylength) — The per-tile size, in bytes, of the persistent threadgroup memory allocation.
- [tileWidth](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/tilewidth) — The tile width, in pixels.
- [tileHeight](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/tileheight) — The tile height, in pixels.
