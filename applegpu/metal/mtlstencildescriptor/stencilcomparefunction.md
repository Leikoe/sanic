# stencilCompareFunction

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilcomparefunction>

The comparison that is performed between the masked reference value and a masked value in the stencil attachment.

## Declaration

```swift
var stencilCompareFunction: MTLCompareFunction { get set }
```

## Discussion

For example, if `stencilCompareFunction` is [MTLCompareFunction.less](https://developer.apple.com/documentation/metal/mtlcomparefunction/less), then the stencil test passes if the masked reference value is less than the masked stored stencil value. The default value is [MTLCompareFunction.always](https://developer.apple.com/documentation/metal/mtlcomparefunction/always), which indicates that the stencil test always passes.

The stored stencil value and the reference value are both *masked* by performing a logical AND operation with the [readMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/readmask) value before the comparison takes place. For more information on possible values, see [MTLCompareFunction](https://developer.apple.com/documentation/metal/mtlcomparefunction).

## See also

### Related Documentation
- [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalue(_:)) — Configures the same comparison value for front- and back-facing primitives.

### Configuring stencil functions and operations
- [stencilFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilfailureoperation) — The operation that is performed to update the values in the stencil attachment when the stencil test fails.
- [depthFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/depthfailureoperation) — The operation that is performed to update the values in the stencil attachment when the stencil test passes, but the depth test fails.
- [depthStencilPassOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/depthstencilpassoperation) — The operation that is performed to update the values in the stencil attachment when both the stencil test and the depth test pass.
- [MTLStencilOperation](https://developer.apple.com/documentation/metal/mtlstenciloperation) — The operation performed on a currently stored stencil value when a comparison test passes or fails.
