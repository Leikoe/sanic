# backFaceStencil

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/backfacestencil>

The stencil descriptor for back-facing primitives.

## Declaration

```swift
@NSCopying var backFaceStencil: MTLStencilDescriptor! { get set }
```

## Discussion

The default value is `nil`, which indicates the stencil test is disabled for the back-facing primitives. For more information, see [MTLStencilDescriptor](https://developer.apple.com/documentation/metal/mtlstencildescriptor).

## See also

### Specifying stencil descriptors for primitives
- [frontFaceStencil](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/frontfacestencil) — The stencil descriptor for front-facing primitives.
