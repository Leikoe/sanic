# setObjectSamplerState(_:lodMinClamp:lodMaxClamp:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstate(_:lodminclamp:lodmaxclamp:index:)>

Assigns a sampler state and clamp values to an entry in the object shader argument table.

## Declaration

```swift
func setObjectSamplerState(_ sampler: (any MTLSamplerState)?, lodMinClamp: Float, lodMaxClamp: Float, index: Int)
```

## Parameters

- **sampler** — An [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance the command assigns to an entry in the object shader argument table for sampler states.
- **lodMinClamp** — The smallest level of detail value an object shader can use when it samples a texture.
- **lodMaxClamp** — The largest level of detail value an object shader can use when it samples a texture.
- **index** — An integer that represents the entry in the object argument table for sampler states that stores a record of `sampler`.

## Discussion

The method’s `lodMinClamp` and `lodMaxClamp` parameters override the default values for `sampler`. You can set the sampler’s default values by configuring the [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) and [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) properties of [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) before you create the sampler.

By default, the sampler state at each index is `nil`.

## See also

### Assigning sampler states for object shaders
- [setObjectSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstate(_:index:)) — Assigns a sampler state to an entry in the object shader argument table.
- [setObjectSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the object shader argument table.
- [setObjectSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the object shader argument table.
