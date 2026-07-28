# MTLSamplerReductionMode

*Enumeration · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlsamplerreductionmode>

Configures how the sampler aggregates contributing samples to a final value.

## Declaration

```swift
enum MTLSamplerReductionMode
```

## Topics

### Enumeration Cases
- [MTLSamplerReductionMode.maximum](https://developer.apple.com/documentation/metal/mtlsamplerreductionmode/maximum) — A reduction mode that finds the maximum contributing sample value by separately evaluating each channel.
- [MTLSamplerReductionMode.minimum](https://developer.apple.com/documentation/metal/mtlsamplerreductionmode/minimum) — A reduction mode that finds the minimum contributing sample value by separately evaluating each channel.
- [MTLSamplerReductionMode.weightedAverage](https://developer.apple.com/documentation/metal/mtlsamplerreductionmode/weightedaverage) — A reduction mode that adds together the product of each contributing sample value by its weight.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlsamplerreductionmode/init(rawvalue:))

## See also

### Texture samplers
- [Creating and sampling textures](https://developer.apple.com/documentation/metal/creating-and-sampling-textures) — Load image data into a texture and apply it to a quadrangle.
- [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) — An instance that defines how a texture should be sampled.
- [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) — An object that you use to configure a texture sampler.
- [MTLSamplePosition](https://developer.apple.com/documentation/metal/mtlsampleposition) — A subpixel sample position for use in multisample antialiasing (MSAA).
