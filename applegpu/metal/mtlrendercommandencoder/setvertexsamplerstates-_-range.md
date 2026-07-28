# setVertexSamplerStates(_:range:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 8.0, macOS 10.11, tvOS 8.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates(_:range:)>

Assigns multiple sampler states to a range of entries in the vertex shader argument table.

## Declaration

```swift
func setVertexSamplerStates(_ samplers: [(any MTLSamplerState)?], range: Range<Int>)
```

## Parameters

- **samplers** — An array of [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instances the command assigns to entries in the vertex shader argument table for sampler states.
- **range** — A span of integers that represent the entries in the vertex shader argument table for sampler states. Each entry stores a record of the corresponding element in `samplers`.

## Discussion

By default, the sampler state at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setVertexSamplerStates:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates:withrange:).

## See also

### Assigning sampler states
- [setVertexSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstate(_:index:)) — Assigns a sampler state to an entry in the vertex shader argument table.
- [setVertexSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the vertex shader argument table.
- [setVertexSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the vertex shader argument table.
