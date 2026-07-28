# dispatchThreadsPerTile(_:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/dispatchthreadspertile(_:)>

Encodes a command that invokes GPU functions from the encoder’s current tile render pipeline state.

## Declaration

```swift
func dispatchThreadsPerTile(_ threadsPerTile: MTLSize)
```

## Parameters

- **threadsPerTile** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads the render pass uses per tile. Set the size’s [width](https://developer.apple.com/documentation/metal/mtlsize/width) and [height](https://developer.apple.com/documentation/metal/mtlsize/height) properties to values that are less than or equal to [tileWidth](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/tilewidth) and [tileHeight](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/tileheight), respectively. Some GPU families only support square tile dispatches and require the same value for [width](https://developer.apple.com/documentation/metal/mtlsize/width) and [height](https://developer.apple.com/documentation/metal/mtlsize/height). See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check which GPU families support nonsquare dispatches. Set the [depth](https://developer.apple.com/documentation/metal/mtlsize/depth) property to `1`.

## Discussion

The command invokes the GPU function that’s in the encoder’s current tile render pipeline state. You can configure that state with the following steps:

1. Configure an [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) instance.

2. Create a tile render pipeline state by calling one of the applicable methods of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance, including [makeRenderPipelineState(tileDescriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:reflection:)).

3. Apply that tile render pipeline state by calling the [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setrenderpipelinestate(_:)) method.

The method records the encoder’s current rendering state and resources the command needs as it runs. You can safely change the encoder’s render pipeline state to encode other commands after calling this method. Subsequent changes to the state don’t affect the commands already in the encoder’s [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer).

## See also

### Drawing with tile shaders
- [tileWidth](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/tilewidth) — The width of the tiles, in pixels, for the render command encoder.
- [tileHeight](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/tileheight) — The height of the tiles, in pixels, for the render command encoder.
