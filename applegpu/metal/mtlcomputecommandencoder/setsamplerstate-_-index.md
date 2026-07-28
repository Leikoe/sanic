# setSamplerState(_:index:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstate(_:index:)>

Encodes a texture sampler, allowing compute kernels to use it for sampling textures on the GPU.

## Declaration

```swift
func setSamplerState(_ sampler: (any MTLSamplerState)?, index: Int)
```

## Parameters

- **sampler** — An [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance to bind to the sampler argument table.
- **index** — The index in the sampler argument table to bind the sampler to.

## See also

### Binding texture samplers
- [setSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Encodes a texture sampler with a custom level of detail clamping, allowing compute kernels to use it for sampling textures on the GPU.
- [setSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstates(_:range:)) — Encodes multiple texture samplers to the sampler argument table, allowing compute kernels to use them for sampling textures on the GPU.
- [setSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Encodes multiple texture samplers for the compute function, specifying clamp values for the level of detail of each sampler.
