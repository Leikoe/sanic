# buffer(for:)

*Instance Method · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensorbufferattachments/buffer(for:)>

Returns the buffer backing the given plane, or `nil` if none has been set.

## Declaration

```swift
func buffer(for plane: MTLTensorPlaneType) -> (any MTLBuffer)?
```

## Parameters

- **plane** — The plane type to look up.

## Return Value

The buffer for the given plane, or `nil`.
