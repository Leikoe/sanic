# setSamplerStates(_:range:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 11.0, macOS 10.13, tvOS 11.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setsamplerstates(_:range:)>

Encodes an array of samplers into the argument buffer.

## Declaration

```swift
func setSamplerStates(_ samplers: [(any MTLSamplerState)?], range: Range<Int>)
```

## Parameters

- **samplers** — An array of samplers the method encodes.
- **range** — A range of indices within the argument buffer for each element in `samplers`. The values correspond to either the index IDs of declarations in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances.

## See also

### Encoding samplers
- [setSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setsamplerstate(_:index:)) — Encodes a sampler into the argument buffer.
