# commit()

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandbuffer/commit()>

Submits the command buffer to the queue for execution on the GPU.

## Declaration

```swift
func commit()
```

## Discussion

If you haven’t already called [enqueue()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/enqueue()) for the command buffer, the [commit()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/commit()) method enqueues it at the next position in the input/output command queue.

You can only commit an input/output command buffer once, after which you can’t encode any additional commands or add more completion handlers to it.

## See also

### Submitting a command buffer
- [enqueue()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/enqueue()) — Reserves a place for the input/output command buffer in the input/output command queue without committing the command buffer.
