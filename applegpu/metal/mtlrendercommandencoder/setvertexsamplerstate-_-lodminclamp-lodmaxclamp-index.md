# setVertexSamplerState(_:lodMinClamp:lodMaxClamp:index:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstate(_:lodminclamp:lodmaxclamp:index:)>

Assigns a sampler state and clamp values to an entry in the vertex shader argument table.

## Declaration

```swift
func setVertexSamplerState(_ sampler: (any MTLSamplerState)?, lodMinClamp: Float, lodMaxClamp: Float, index: Int)
```

## Parameters

- **sampler** — An [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance the command assigns to an entry in the vertex shader argument table for sampler states.
- **lodMinClamp** — The smallest level of detail value a vertex shader can use when it samples a texture.
- **lodMaxClamp** — The largest level of detail value a vertex shader can use when it samples a texture.
- **index** — An integer that represents the entry in the vertex shader argument table for sampler states that stores a record of `sampler`.

## Discussion

The method’s `lodMinClamp` and `lodMaxClamp` parameters override the default values for `sampler`. You can set the sampler’s default values by configuring the [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) and [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) properties of [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) before you create the sampler.

By default, the sampler state at each index is `nil`.

## See also

### Assigning sampler states
- [setVertexSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstate(_:index:)) — Assigns a sampler state to an entry in the vertex shader argument table.
- [setVertexSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the vertex shader argument table.
- [setVertexSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the vertex shader argument table.
