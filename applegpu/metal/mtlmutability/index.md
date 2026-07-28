# MTLMutability

*Enumeration · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlmutability>

The options that determine the mutability of a buffer’s contents.

## Declaration

```swift
enum MTLMutability
```

## Topics

### Enumeration cases
- [MTLMutability.default](https://developer.apple.com/documentation/metal/mtlmutability/default) — The default behavior, based on the buffer’s type.
- [MTLMutability.mutable](https://developer.apple.com/documentation/metal/mtlmutability/mutable) — An option that states that you can modify the buffer’s contents.
- [MTLMutability.immutable](https://developer.apple.com/documentation/metal/mtlmutability/immutable) — An option that states that you can’t modify the buffer’s contents.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlmutability/init(rawvalue:))

## See also

### Setting buffer mutability
- [mutability](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor/mutability) — A mutability option that determines whether you can update a buffer’s contents before related commands use the buffer.
