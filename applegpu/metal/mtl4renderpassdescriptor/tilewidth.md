# tileWidth

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4renderpassdescriptor/tilewidth>

The width of the tiles, in pixels, a render pass you create with this descriptor applies to its attachments.

## Declaration

```swift
var tileWidth: Int { get set }
```

## Discussion

For tile-based rendering, Metal divides each render attachment into smaller regions, or *tiles*. The property’s default is `0`, which tells Metal to select a size that fits in tile memory.

See [Tailor your apps for Apple GPUs and tile-based deferred rendering](https://developer.apple.com/documentation/metal/tailor-your-apps-for-apple-gpus-and-tile-based-deferred-rendering) for more information about tiles, tile memory, and deferred rendering.
