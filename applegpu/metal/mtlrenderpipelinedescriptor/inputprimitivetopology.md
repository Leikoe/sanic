# inputPrimitiveTopology

*Instance Property · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.11, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/inputprimitivetopology>

The type of primitive topology the pipeline renders.

## Declaration

```swift
var inputPrimitiveTopology: MTLPrimitiveTopologyClass { get set }
```

## Discussion

Your app needs to specify this value when layered rendering is enabled.

The default value is `MTLPrimitiveTopologyClassUnspecified`.

## See also

### Related Documentation
- [renderTargetArrayLength](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/rendertargetarraylength) — The number of active layers that all attachments need to have for layered rendering.

### Specifying rasterization and visibility state
- [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatocoverageenabled) — A Boolean value that indicates whether to read and use the alpha channel fragment output for color attachments to compute a sample coverage mask.
- [isAlphaToOneEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatooneenabled) — A Boolean value that indicates whether to force alpha channel values for color attachments to the largest representable value.
- [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/israsterizationenabled) — A Boolean value that determines whether the pipeline rasterizes primitives.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/rastersamplecount) — The number of samples the pipeline applies for each fragment.
- [MTLPrimitiveTopologyClass](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass) — The primitive topologies available for rendering.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/samplecount) — The number of samples the pipeline applies for each fragment.
