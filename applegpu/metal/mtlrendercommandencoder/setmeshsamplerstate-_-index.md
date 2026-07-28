# setMeshSamplerState(_:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstate(_:index:)>

Assigns a sampler state to an entry in the mesh shader argument table.

## Declaration

```swift
func setMeshSamplerState(_ sampler: (any MTLSamplerState)?, index: Int)
```

## Parameters

- **sampler** — An [MTLSamplerState](https://developer.apple.com/documentation/metal/mtlsamplerstate) instance the command assigns to an entry in the mesh shader argument table for sampler states.
- **index** — An integer that represents the entry in the mesh shader argument table for sampler states that stores a record of `sampler`.

## Discussion

By default, the sampler state at each index is `nil`.

## See also

### Assigning sampler states for mesh shaders
- [setMeshSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Assigns a sampler state and clamp values to an entry in the mesh shader argument table.
- [setMeshSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstates(_:range:)) — Assigns multiple sampler states to a range of entries in the mesh shader argument table.
- [setMeshSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Assigns multiple sampler states and clamp values to a range of entries in the mesh shader argument table.
