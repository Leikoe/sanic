# setTileSamplerState(_:lodMinClamp:lodMaxClamp:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstate(_:lodminclamp:lodmaxclamp:index:)>

Assigns a sampler state and clamp values to an entry in the tile shader argument table.

## Declaration

```swift
func setTileSamplerState(_ sampler: (any MTLSamplerState)?, lodMinClamp: Float, lodMaxClamp: Float, index: Int)
```

## Parameters

- **sampler** — An [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance the command assigns to an entry in the tile shader argument table for sampler states.
- **lodMinClamp** — The smallest level of detail value a tile shader can use when it samples a texture.
- **lodMaxClamp** — The largest level of detail value a tile shader can use when it samples a texture.
- **index** — An integer that represents the entry in the tile shader argument table for sampler states that stores a record of `sampler`.

## Discussion

The method’s `lodMinClamp` and `lodMaxClamp` parameters override the default values for `sampler`. You can set the sampler’s default values by configuring the [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) and [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) properties of [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) before you create the sampler.

By default, the sampler state at each index is `nil`.

## See also

### Assigning sampler states
- [setTileSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstate(_:index:)) — Assigns a sampler state to an entry in the tile shader argument table.
- [setTileSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the tile shader argument table.
- [setTileSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilesamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the tile shader argument table.
