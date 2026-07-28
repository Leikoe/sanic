# readMask

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstencildescriptor/readmask>

A bitmask that determines from which bits that stencil comparison tests can read.

## Declaration

```swift
var readMask: UInt32 { get set }
```

## Discussion

The [readMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/readmask) bits are used for logical AND operations to both the stored stencil value and the reference value.

The least significant bits of the read mask are used. The default value is all ones. A logical AND operation with the default [readMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/readmask) does not change the value.

## See also

### Related Documentation
- [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalue(_:)) — Configures the same comparison value for front- and back-facing primitives.

### Configuring stencil bit mask properties
- [writeMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/writemask) — A bitmask that determines to which bits that stencil operations can write.
