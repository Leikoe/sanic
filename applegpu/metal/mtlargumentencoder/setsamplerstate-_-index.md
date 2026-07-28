# setSamplerState(_:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setsamplerstate(_:index:)>

Encodes a sampler into the argument buffer.

## Declaration

```swift
func setSamplerState(_ sampler: (any MTLSamplerState)?, index: Int)
```

## Parameters

- **sampler** — A sampler the method encodes.
- **index** — The index of a sampler within the argument buffer. The value corresponds to either the index ID of a declaration in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of an [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instance.

## See also

### Encoding samplers
- [setSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setsamplerstates(_:range:)) — Encodes an array of samplers into the argument buffer.
