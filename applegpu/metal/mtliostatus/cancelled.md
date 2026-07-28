# MTLIOStatus.cancelled

*Case · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliostatus/cancelled>

Indicates the GPU has successfully abandoned the input/output command buffer.

## Declaration

```swift
case cancelled
```

## See also

### I/O command queue states
- [MTLIOStatus.pending](https://developer.apple.com/documentation/metal/mtliostatus/pending) — Indicates the GPU hasn’t finished executing the input/output command buffer.
- [MTLIOStatus.complete](https://developer.apple.com/documentation/metal/mtliostatus/complete) — Indicates the GPU has successfully finished executing the input/output command buffer.
- [MTLIOStatus.error](https://developer.apple.com/documentation/metal/mtliostatus/error) — Indicates the GPU experienced a problem with the input/output command buffer.
