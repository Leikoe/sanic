# addBarrier()

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandbuffer/addbarrier()>

Encodes a barrier into the command buffer.

## Declaration

```swift
func addBarrier()
```

## Discussion

The method encodes a barrier that starts any subsequent commands only after all the previously encoded commands have completed.
