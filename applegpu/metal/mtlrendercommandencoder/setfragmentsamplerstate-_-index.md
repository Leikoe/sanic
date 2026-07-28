# setFragmentSamplerState(_:index:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstate(_:index:)>

Assigns a sampler state to an entry in the fragment shader argument table.

## Declaration

```swift
func setFragmentSamplerState(_ sampler: (any MTLSamplerState)?, index: Int)
```

## Parameters

- **sampler** — An [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance the command assigns to an entry in the fragment shader argument table for sampler states.
- **index** — An integer that represents the entry in the fragment shader argument table for sampler states that stores a record of `sampler`.

## Discussion

By default, the sampler state at each index is `nil`.

## See also

### Assigning sampler states
- [setFragmentSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the fragment shader argument table.
- [setFragmentSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the fragment shader argument table.
- [setFragmentSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the fragment shader argument table.
