# tileWidth

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/tilewidth>

The width of the tiles, in pixels, for the render command encoder.

## Declaration

```swift
var tileWidth: Int { get }
```

## Discussion

The value comes from the [tileWidth](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/tilewidth) property of the [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) at the time you create the render command encoder.

## See also

### Drawing with tile shaders
- [dispatchThreadsPerTile(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/dispatchthreadspertile(_:)) — Encodes a command that invokes GPU functions from the encoder’s current tile render pipeline state.
- [tileHeight](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/tileheight) — The height of the tiles, in pixels, for the render command encoder.
