# setFragmentSamplerStates(_:range:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 8.0, macOS 10.11, tvOS 8.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstates(_:range:)>

Assigns multiple sampler states to a range of entries in the fragment shader argument table.

## Declaration

```swift
func setFragmentSamplerStates(_ samplers: [(any MTLSamplerState)?], range: Range<Int>)
```

## Parameters

- **samplers** — An array of [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instances the command assigns to entries in the fragment shader argument table for sampler states.
- **range** — A span of integers that represent the entries in the fragment shader argument table for sampler states. Each entry stores a record of the corresponding element in `samplers`.

## Discussion

By default, the sampler state at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setFragmentSamplerStates:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstates:withrange:).

## See also

### Assigning sampler states
- [setFragmentSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstate(_:index:)) — Assigns a sampler state to an entry in the fragment shader argument table.
- [setFragmentSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the fragment shader argument table.
- [setFragmentSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the fragment shader argument table.
