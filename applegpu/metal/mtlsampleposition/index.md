# MTLSamplePosition

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlsampleposition>

A subpixel sample position for use in multisample antialiasing (MSAA).

## Declaration

```swift
struct MTLSamplePosition
```

## Overview

Subpixel sample positions are in a 16 x 16 grid across a pixel. Each subsample position’s [x](https://developer.apple.com/documentation/metal/mtlsampleposition/x) and [y](https://developer.apple.com/documentation/metal/mtlsampleposition/y) values are in 1/16 increments in the floating-point range `[0.0, 15.0/16.0)`. The pixel’s origin point `(0,0)` is at the top-left corner.

See [Positioning samples programmatically](https://developer.apple.com/documentation/metal/positioning-samples-programmatically) for the details on working with subpixels.

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtlsampleposition/init()) — Returns a new sample position on a subpixel grid.
- [init(x:y:)](https://developer.apple.com/documentation/metal/mtlsampleposition/init(x:y:)) — Returns a new sample position on a subpixel grid at specified coordinates.

### Instance Properties
- [x](https://developer.apple.com/documentation/metal/mtlsampleposition/x) — The x position of the sample on the subpixel grid.
- [y](https://developer.apple.com/documentation/metal/mtlsampleposition/y) — The y position of the sample on the subpixel grid.

## See also

### Texture samplers
- [Creating and sampling textures](https://developer.apple.com/documentation/metal/creating-and-sampling-textures) — Load image data into a texture and apply it to a quadrangle.
- [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) — An instance that defines how a texture should be sampled.
- [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) — An object that you use to configure a texture sampler.
- [MTLSamplerReductionMode](https://developer.apple.com/documentation/metal/mtlsamplerreductionmode) — Configures how the sampler aggregates contributing samples to a final value.
