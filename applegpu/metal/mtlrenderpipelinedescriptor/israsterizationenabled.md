# isRasterizationEnabled

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/israsterizationenabled>

A Boolean value that determines whether the pipeline rasterizes primitives.

## Declaration

```swift
var isRasterizationEnabled: Bool { get set }
```

## Discussion

The default value is [true](https://developer.apple.com/documentation/Swift/true), indicating that primitives are rasterized. If the value is [false](https://developer.apple.com/documentation/Swift/false), then primitives are dropped prior to rasterization (i.e. rasterization is disabled). Disabling rasterization may be useful to gather data from vertex-only transformations.

When this value is [false](https://developer.apple.com/documentation/Swift/false), no fragments are processed and the vertex shader function needs to return `void`.

## See also

### Specifying rasterization and visibility state
- [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatocoverageenabled) — A Boolean value that indicates whether to read and use the alpha channel fragment output for color attachments to compute a sample coverage mask.
- [isAlphaToOneEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatooneenabled) — A Boolean value that indicates whether to force alpha channel values for color attachments to the largest representable value.
- [inputPrimitiveTopology](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/inputprimitivetopology) — The type of primitive topology the pipeline renders.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/rastersamplecount) — The number of samples the pipeline applies for each fragment.
- [MTLPrimitiveTopologyClass](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass) — The primitive topologies available for rendering.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/samplecount) — The number of samples the pipeline applies for each fragment.
