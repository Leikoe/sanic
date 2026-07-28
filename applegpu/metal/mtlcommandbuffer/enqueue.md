# enqueue()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()>

Reserves the next available place for the command buffer in its command queue.

## Declaration

```swift
func enqueue()
```

## Discussion

The [enqueue()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()) method adds the command buffer to the [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance that owns it, but doesn’t commit the command buffer to run on the GPU. You can call the command buffer’s [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method at a later time when it’s ready to run on the GPU. You can call a command buffer’s [enqueue()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()) method any time before you call [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()), including before, after, or as you encode commands to it.

> **Note:**
>  The command buffer can only reserve a place in its queue a single time; all subsequent [enqueue()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()) calls have no effect.

Enqueuing your command buffers first gives you the flexibility to arrange their relative order of execution before encoding commands to any of them. This approach lets you potentially encode each command buffer on a thread, in parallel, instead of encoding them one by one on a single thread. The order in which each worker thread finishes encoding and commits its command buffer doesn’t matter when you enqueue them in order before committing.

## See also

### Submitting a command buffer
- [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) — Submits the command buffer to run on the GPU.
