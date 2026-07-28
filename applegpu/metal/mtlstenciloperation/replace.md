# MTLStencilOperation.replace

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstenciloperation/replace>

A stencil operation that replaces a stencil value with a reference value.

## Declaration

```swift
case replace
```

## Discussion

You can set by the reference value by calling the [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalue(_:)) method of an [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instance.

## See also

### Stencil operations
- [MTLStencilOperation.keep](https://developer.apple.com/documentation/metal/mtlstenciloperation/keep) — A stencil operation that doesn’t modify a stencil value.
- [MTLStencilOperation.zero](https://developer.apple.com/documentation/metal/mtlstenciloperation/zero) — A stencil operation that sets a stencil value to zero.
- [MTLStencilOperation.incrementClamp](https://developer.apple.com/documentation/metal/mtlstenciloperation/incrementclamp) — A stencil operation that increases a stencil value by one, but only when the current value isn’t the maximum representable value.
- [MTLStencilOperation.decrementClamp](https://developer.apple.com/documentation/metal/mtlstenciloperation/decrementclamp) — A stencil operation that decreases a nonzero stencil value by one.
- [MTLStencilOperation.invert](https://developer.apple.com/documentation/metal/mtlstenciloperation/invert) — A stencil operation that applies a logical bitwise NOT to a stencil value.
- [MTLStencilOperation.incrementWrap](https://developer.apple.com/documentation/metal/mtlstenciloperation/incrementwrap) — A stencil operation that decreases a nonzero stencil value by one, or when it’s the maximum representable value, resets it to zero.
- [MTLStencilOperation.decrementWrap](https://developer.apple.com/documentation/metal/mtlstenciloperation/decrementwrap) — A stencil operation that decreases a nonzero stencil value by one, or when it’s zero, resets it to the maximum representable value.
