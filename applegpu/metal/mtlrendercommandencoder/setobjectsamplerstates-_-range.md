# setObjectSamplerStates(_:range:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates(_:range:)>

Assigns multiple sampler states to a range of entries in the object shader argument table.

## Declaration

```swift
func setObjectSamplerStates(_ samplers: [(any MTLSamplerState)?], range: Range<Int>)
```

## Parameters

- **samplers** — An array of [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instances the command assigns to entries in the object shader argument table for sampler states.
- **range** — A span of integers that represent the entries in the object shader argument table for sampler states. Each entry stores a record of the corresponding element in `samplers`.

## Discussion

By default, the sampler state at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setObjectSamplerStates:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates:withrange:).

## See also

### Assigning sampler states for object shaders
- [setObjectSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstate(_:index:)) — Assigns a sampler state to an entry in the object shader argument table.
- [setObjectSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the object shader argument table.
- [setObjectSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the object shader argument table.
