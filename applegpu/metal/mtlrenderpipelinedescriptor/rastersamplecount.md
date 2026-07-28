# rasterSampleCount

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/rastersamplecount>

The number of samples the pipeline applies for each fragment.

## Declaration

```swift
var rasterSampleCount: Int { get set }
```

## Discussion

The render pipeline state honors this property only if the pipeline render targets support multisampling.

> **Important:**
>  This property needs to be `1` if the render targets don’t support multisampling.

When your create an [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instance, this property’s value needs to be equal to the number of render target textures. Furthermore, the texture type of all render target textures need to be [MTLTextureType.type2DMultisample](https://developer.apple.com/documentation/metal/mtltexturetype/type2dmultisample).

The number of samples a GPU supports varies by device. You can check whether an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance supports a specific sample count by calling its [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) method.

The default value for this property is `1`.

## See also

### Specifying rasterization and visibility state
- [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatocoverageenabled) — A Boolean value that indicates whether to read and use the alpha channel fragment output for color attachments to compute a sample coverage mask.
- [isAlphaToOneEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatooneenabled) — A Boolean value that indicates whether to force alpha channel values for color attachments to the largest representable value.
- [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/israsterizationenabled) — A Boolean value that determines whether the pipeline rasterizes primitives.
- [inputPrimitiveTopology](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/inputprimitivetopology) — The type of primitive topology the pipeline renders.
- [MTLPrimitiveTopologyClass](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass) — The primitive topologies available for rendering.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/samplecount) — The number of samples the pipeline applies for each fragment.
