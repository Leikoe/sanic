# dispatchThreadsPerTile(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/dispatchthreadspertile(_:)>

Encodes a command that invokes a tile shader function from the encoder’s current tile render pipeline state.

## Declaration

```swift
func dispatchThreadsPerTile(_ threadsPerTile: MTLSize)
```

## Parameters

- **threadsPerTile** — A [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads the render pass uses per tile. Set the size’s [width](https://developer.apple.com/documentation/metal/mtlsize/width) and [height](https://developer.apple.com/documentation/metal/mtlsize/height) properties to values that are less than or equal to [tileWidth](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/tilewidth) and [tileHeight](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/tileheight), respectively. Some GPU families only support square tile dispatches and require the same value for width and height. Set [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) to `1`.

## See also

### Drawing with tile shaders
- [tileWidth](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/tilewidth) — Sets the width of a tile for this render pass.
- [tileHeight](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/tileheight) — Sets the height of a tile for this render pass.
