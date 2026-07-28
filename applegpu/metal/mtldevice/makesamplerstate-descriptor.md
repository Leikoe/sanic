# makeSamplerState(descriptor:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makesamplerstate(descriptor:)>

Creates a sampler state instance.

## Declaration

```swift
func makeSamplerState(descriptor: MTLSamplerDescriptor) -> (any MTLSamplerState)?
```

## Parameters

- **descriptor** — An [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) instance.

## Return Value

A new [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance if the method completed successfully; otherwise `nil`.

## See also

### Creating samplers
- [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) — Returns a Boolean value that indicates whether the GPU can sample a texture with a specific number of sample points.
- [getDefaultSamplePositions(sampleCount:)](https://developer.apple.com/documentation/metal/mtldevice/getdefaultsamplepositions(samplecount:)) — Returns the default sample locations based on the number of samples.
