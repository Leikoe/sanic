# mutability

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor/mutability>

A mutability option that determines whether you can update a buffer’s contents before related commands use the buffer.

## Declaration

```swift
var mutability: MTLMutability { get set }
```

## Discussion

The default value is [MTLMutability.default](https://developer.apple.com/documentation/metal/mtlmutability/default).

If you don’t explicitly declare mutability, Metal uses the following default behaviors:

- Regular buffers are mutable by default, and Metal treats [MTLMutability.default](https://developer.apple.com/documentation/metal/mtlmutability/default) as if it were [MTLMutability.mutable](https://developer.apple.com/documentation/metal/mtlmutability/mutable).

- Argument buffers are immutable by default, and Metal treats [MTLMutability.default](https://developer.apple.com/documentation/metal/mtlmutability/default) as if it were [MTLMutability.immutable](https://developer.apple.com/documentation/metal/mtlmutability/immutable).

## See also

### Setting buffer mutability
- [MTLMutability](https://developer.apple.com/documentation/metal/mtlmutability) — The options that determine the mutability of a buffer’s contents.
