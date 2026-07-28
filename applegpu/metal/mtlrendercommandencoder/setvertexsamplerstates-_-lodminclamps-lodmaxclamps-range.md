# setVertexSamplerStates(_:lodMinClamps:lodMaxClamps:range:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 8.0, macOS 10.11, tvOS 8.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates(_:lodminclamps:lodmaxclamps:range:)>

Assigns multiple sampler states and clamp values to a range of entries in the vertex shader argument table.

## Declaration

```swift
func setVertexSamplerStates(_ samplers: [(any MTLSamplerState)?], lodMinClamps: [Float], lodMaxClamps: [Float], range: Range<Int>)
```

## Parameters

- **samplers** — An array of [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instances the command assigns to entries in the vertex shader argument table for sampler states.
- **lodMinClamps** — An array of floating-point values. Each element is the smallest level of detail value a vertex shader can use when it samples a texture with the corresponding element in `samplers`.
- **lodMaxClamps** — An array of floating-point values. Each element is the largest level of detail value a vertex shader can use when it samples a texture with the corresponding element in `samplers`.
- **range** — A span of integers that represent the entries in the vertex shader argument table for sampler states. Each entry stores a record of the corresponding element in `samplers`.

## Discussion

Each element of the method’s `lodMinClamps` and `lodMaxClamps` parameters overrides the default values for the corresponding sampler in `samplers`. You can set a sampler’s default values by configuring the [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) and [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) properties of [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) before you create the sampler.

By default, the sampler state at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setVertexSamplerStates:lodMinClamps:lodMaxClamps:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates:lodminclamps:lodmaxclamps:withrange:).

## See also

### Assigning sampler states
- [setVertexSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstate(_:index:)) — Assigns a sampler state to an entry in the vertex shader argument table.
- [setVertexSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the vertex shader argument table.
- [setVertexSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the vertex shader argument table.
