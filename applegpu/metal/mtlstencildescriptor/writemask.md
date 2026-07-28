# writeMask

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstencildescriptor/writemask>

A bitmask that determines to which bits that stencil operations can write.

## Declaration

```swift
var writeMask: UInt32 { get set }
```

## Discussion

[writeMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/writemask) are used for logical AND operations to values that are going to be written into a stencil attachment as the result of a stencil operation.

The least significant bits of the write mask are used. The default value is all ones. A logical AND operation with the default [writeMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/writemask) does not change the value.

## See also

### Configuring stencil bit mask properties
- [readMask](https://developer.apple.com/documentation/metal/mtlstencildescriptor/readmask) — A bitmask that determines from which bits that stencil comparison tests can read.
