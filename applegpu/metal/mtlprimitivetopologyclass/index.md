# MTLPrimitiveTopologyClass

*Enumeration · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.11, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass>

The primitive topologies available for rendering.

## Declaration

```swift
enum MTLPrimitiveTopologyClass
```

## Topics

### Topology classes
- [MTLPrimitiveTopologyClass.unspecified](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass/unspecified) — An unspecified primitive.
- [MTLPrimitiveTopologyClass.point](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass/point) — A point primitive.
- [MTLPrimitiveTopologyClass.line](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass/line) — A line primitive.
- [MTLPrimitiveTopologyClass.triangle](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass/triangle) — A triangle primitive.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass/init(rawvalue:))

## See also

### Specifying rasterization and visibility state
- [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatocoverageenabled) — A Boolean value that indicates whether to read and use the alpha channel fragment output for color attachments to compute a sample coverage mask.
- [isAlphaToOneEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatooneenabled) — A Boolean value that indicates whether to force alpha channel values for color attachments to the largest representable value.
- [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/israsterizationenabled) — A Boolean value that determines whether the pipeline rasterizes primitives.
- [inputPrimitiveTopology](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/inputprimitivetopology) — The type of primitive topology the pipeline renders.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/rastersamplecount) — The number of samples the pipeline applies for each fragment.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/samplecount) — The number of samples the pipeline applies for each fragment.
