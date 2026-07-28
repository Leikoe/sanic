# setTileSamplerStates(_:range:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstates(_:range:)>

Assigns multiple sampler states to a range of entries in the tile shader argument table.

## Declaration

```swift
func setTileSamplerStates(_ samplers: [(any MTLSamplerState)?], range: Range<Int>)
```

## Parameters

- **samplers** — An array of [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instances the command assigns to entries in the tile shader argument table for sampler states.
- **range** — A span of integers that represent the entries in the tile shader argument table for sampler states. Each entry stores a record of the corresponding element in `samplers`.

## Discussion

By default, the sampler state at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setTileSamplerStates:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstates:withrange:).

## See also

### Assigning sampler states
- [setTileSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstate(_:index:)) — Assigns a sampler state to an entry in the tile shader argument table.
- [setTileSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the tile shader argument table.
- [setTileSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the tile shader argument table.
