# stencilFailureOperation

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilfailureoperation>

The operation that is performed to update the values in the stencil attachment when the stencil test fails.

## Declaration

```swift
var stencilFailureOperation: MTLStencilOperation { get set }
```

## Discussion

The default value is [MTLStencilOperation.keep](https://developer.apple.com/documentation/metal/mtlstenciloperation/keep), which does not change the current stencil value. For more information on possible values, see [MTLStencilOperation](https://developer.apple.com/documentation/metal/mtlstenciloperation).

When the stencil test fails for a pixel, its incoming color, depth, or stencil values are discarded.

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Configuring stencil functions and operations
- [depthFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/depthfailureoperation) — The operation that is performed to update the values in the stencil attachment when the stencil test passes, but the depth test fails.
- [depthStencilPassOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/depthstencilpassoperation) — The operation that is performed to update the values in the stencil attachment when both the stencil test and the depth test pass.
- [stencilCompareFunction](https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilcomparefunction) — The comparison that is performed between the masked reference value and a masked value in the stencil attachment.
- [MTLStencilOperation](https://developer.apple.com/documentation/metal/mtlstenciloperation) — The operation performed on a currently stored stencil value when a comparison test passes or fails.
