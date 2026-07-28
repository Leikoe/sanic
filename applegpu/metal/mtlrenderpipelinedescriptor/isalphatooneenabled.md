# isAlphaToOneEnabled

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatooneenabled>

A Boolean value that indicates whether to force alpha channel values for color attachments to the largest representable value.

## Declaration

```swift
var isAlphaToOneEnabled: Bool { get set }
```

## Discussion

The default value is [false](https://developer.apple.com/documentation/Swift/false).

If enabled, alpha channel fragment values are only forced for `colorAttachments[0]`. Other attachments are unaffected.

You may use `alphaToOneEnabled` when you want to write an alpha value that represents partial coverage of the pixel, but also want to disable blending (by forcing alpha to one).

## See also

### Specifying rasterization and visibility state
- [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatocoverageenabled) — A Boolean value that indicates whether to read and use the alpha channel fragment output for color attachments to compute a sample coverage mask.
- [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/israsterizationenabled) — A Boolean value that determines whether the pipeline rasterizes primitives.
- [inputPrimitiveTopology](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/inputprimitivetopology) — The type of primitive topology the pipeline renders.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/rastersamplecount) — The number of samples the pipeline applies for each fragment.
- [MTLPrimitiveTopologyClass](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass) — The primitive topologies available for rendering.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/samplecount) — The number of samples the pipeline applies for each fragment.
