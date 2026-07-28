# setObjectSamplerStates(_:lodMinClamps:lodMaxClamps:range:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates(_:lodminclamps:lodmaxclamps:range:)>

Assigns multiple sampler states and clamp values to a range of entries in the object shader argument table.

## Declaration

```swift
func setObjectSamplerStates(_ samplers: [(any MTLSamplerState)?], lodMinClamps: [Float], lodMaxClamps: [Float], range: Range<Int>)
```

## Parameters

- **samplers** — An array of [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instances the command assigns to entries in the object shader argument table for sampler states.
- **lodMinClamps** — An array of floating-point values. Each element is the smallest level of detail value an object shader can use when it samples a texture with the corresponding element in `samplers`.
- **lodMaxClamps** — An array of floating-point values. Each element is the largest level of detail value an object shader can use when it samples a texture with the corresponding element in `samplers`.
- **range** — A span of integers that represent the entries in the object shader argument table for sampler states. Each entry stores a record of the corresponding element in `samplers`.

## Discussion

Each element of the method’s `lodMinClamps` and `lodMaxClamps` parameters overrides the default values for the corresponding sampler in `samplers`. You can set a sampler’s default values by configuring the [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodminclamp) and [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/lodmaxclamp) properties of [MTLSamplerDescriptor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor) before you create the sampler.

By default, the sampler state at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setObjectSamplerStates:lodMinClamps:lodMaxClamps:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates:lodminclamps:lodmaxclamps:withrange:).

## See also

### Assigning sampler states for object shaders
- [setObjectSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstate(_:index:)) — Assigns a sampler state to an entry in the object shader argument table.
- [setObjectSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the object shader argument table.
- [setObjectSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the object shader argument table.
