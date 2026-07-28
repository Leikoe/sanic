# setSamplerStates(_:range:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 8.0, macOS 10.11, tvOS 8.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstates(_:range:)>

Encodes multiple texture samplers to the sampler argument table, allowing compute kernels to use them for sampling textures on the GPU.

## Declaration

```swift
func setSamplerStates(_ samplers: [(any MTLSamplerState)?], range: Range<Int>)
```

## Parameters

- **samplers** — An array of [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance to bind to the sampler argument table.
- **range** — The sampler table indicies to bind each of the `samplers` to, in the order they appear.

## Discussion

> **Warning:**
>  This method requires that the number of instances in `samplers` be the same as the length of `range`.

## See also

### Binding texture samplers
- [setSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstate(_:index:)) — Encodes a texture sampler, allowing compute kernels to use it for sampling textures on the GPU.
- [setSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Encodes a texture sampler with a custom level of detail clamping, allowing compute kernels to use it for sampling textures on the GPU.
- [setSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Encodes multiple texture samplers for the compute function, specifying clamp values for the level of detail of each sampler.
