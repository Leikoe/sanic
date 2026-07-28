# MTLSamplerState

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsamplerstate>

An instance that defines how a texture should be sampled.

## Declaration

```swift
protocol MTLSamplerState : NSObjectProtocol, Sendable
```

## Overview

The [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) protocol defines the interface for a lightweight instance used to encode how a shader or compute kernel should sample a texture. To create a sampler state instance:

1. Create an [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) instance.

2. Set the desired properties of the sampler descriptor, including filtering options, addressing modes, maximum anisotropy, and level-of-detail parameters.

3. Call the [makeSamplerState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesamplerstate(descriptor:)) method of the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

(Your app does not define a class that implements the [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) protocol.)

You can either release the [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) instance or modify its property values and reuse it to create more [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instances. The descriptor’s properties are only used during instance creation; once created the behavior of a sampler state instance is fixed and cannot be changed.

## Topics

### Identifying the sampler
- [device](https://developer.apple.com/documentation/metal/mtlsamplerstate/device) — The device object that created the sampler.
- [label](https://developer.apple.com/documentation/metal/mtlsamplerstate/label) — A string that identifies the sampler.

### Instance Properties
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlsamplerstate/gpuresourceid)

## See also

### Texture samplers
- [Creating and sampling textures](https://developer.apple.com/documentation/metal/creating-and-sampling-textures) — Load image data into a texture and apply it to a quadrangle.
- [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) — An object that you use to configure a texture sampler.
- [MTLSamplePosition](https://developer.apple.com/documentation/metal/mtlsampleposition) — A subpixel sample position for use in multisample antialiasing (MSAA).
- [MTLSamplerReductionMode](https://developer.apple.com/documentation/metal/mtlsamplerreductionmode) — Configures how the sampler aggregates contributing samples to a final value.
