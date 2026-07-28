# supportsTextureSampleCount(_:)

*Instance Method · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)>

Returns a Boolean value that indicates whether the GPU can sample a texture with a specific number of sample points.

## Declaration

```swift
func supportsTextureSampleCount(_ sampleCount: Int) -> Bool
```

## Parameters

- **sampleCount** — The number of points a GPU can sample from a texture.

## Discussion

The number of points the GPU can sample a texture varies by device:

| Sample count | Devices |
|---|---|
| 1 | All devices |
| 2 | All iOS devices ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) All tvOS devices ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) Some macOS devices |
| 4 | All devices |
| 8 | Some macOS devices |

Consider a GPU device’s limitations for sample count by checking [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture)`.`[sampleCount](https://developer.apple.com/documentation/metal/mtltexture/samplecount) when configuring these properties:

- [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor)`.`[sampleCount](https://developer.apple.com/documentation/metal/mtltexturedescriptor/samplecount)

- [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor)`.`[rasterSampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/rastersamplecount)

- [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor)`.`[rasterSampleCount](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/rastersamplecount)

- [MTLMeshRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor)`.`[rasterSampleCount](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/rastersamplecount)

- [MTKView](https://developer.apple.com/documentation/MetalKit/MTKView)`.`[sampleCount](https://developer.apple.com/documentation/MetalKit/MTKView/sampleCount)

## See also

### Creating samplers
- [makeSamplerState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesamplerstate(descriptor:)) — Creates a sampler state instance.
- [getDefaultSamplePositions(sampleCount:)](https://developer.apple.com/documentation/metal/mtldevice/getdefaultsamplepositions(samplecount:)) — Returns the default sample locations based on the number of samples.
