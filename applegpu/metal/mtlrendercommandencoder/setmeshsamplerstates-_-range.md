# setMeshSamplerStates(_:range:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstates(_:range:)>

Assigns multiple sampler states to a range of entries in the mesh shader argument table.

## Declaration

```swift
func setMeshSamplerStates(_ samplers: [(any MTLSamplerState)?], range: Range<Int>)
```

## Parameters

- **samplers** — An array of [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instances the command assigns to entries in the mesh shader argument table for sampler states.
- **range** — A span of integers that represent the entries in the mesh shader argument table for sampler states. Each entry stores a record of the corresponding element in `samplers`.

## Discussion

By default, the sampler state at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setMeshSamplerStates:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstates:withrange:).

## See also

### Assigning sampler states for mesh shaders
- [setMeshSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstate(_:index:)) — Assigns a sampler state to an entry in the mesh shader argument table.
- [setMeshSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the mesh shader argument table.
- [setMeshSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the mesh shader argument table.
