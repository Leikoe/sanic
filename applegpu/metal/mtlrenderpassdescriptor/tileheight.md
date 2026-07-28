# tileHeight

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/tileheight>

The tile height, in pixels.

## Declaration

```swift
var tileHeight: Int { get set }
```

## Discussion

The valid tile sizes are `32 x 32`, `32 x 16`, and `16 x 16`. The Metal driver chooses a default size when your app doesn’t set a tile size.

## See also

### Specifying tile shading parameters
- [imageblockSampleLength](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/imageblocksamplelength) — The per-sample size, in bytes, of the largest explicit imageblock layout in the render pass.
- [threadgroupMemoryLength](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/threadgroupmemorylength) — The per-tile size, in bytes, of the persistent threadgroup memory allocation.
- [tileWidth](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/tilewidth) — The tile width, in pixels.
