# commit()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()>

Submits the command buffer to run on the GPU.

## Declaration

```swift
func commit()
```

## Discussion

The [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method sends the command buffer to the [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance that owns it, which then schedules it to run on the GPU. If your app calls [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) for a command buffer that isn’t enqueued, the method effectively calls [enqueue()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()) for you.

The [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method has several restrictions, including:

- You can commit a command buffer to its command queue only one time.

- You can only commit a command buffer when it doesn’t have an active encoder (see [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) and [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder)).

- You can’t encode additional commands to a command buffer after you commit it.

- You can’t call the [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) or [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) methods after you commit a command buffer.

The GPU starts the command buffer after it starts any command buffers that are ahead of it in the same command queue.

## See also

### Submitting a command buffer
- [enqueue()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()) — Reserves the next available place for the command buffer in its command queue.
