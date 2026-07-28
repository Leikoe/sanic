# barrier(afterQueueStages:beforeStages:visibilityOptions:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterqueuestages:beforestages:visibilityoptions:)>

Encodes a consumer barrier on work you commit to the same command queue.

## Declaration

```swift
func barrier(afterQueueStages: MTLStages, beforeStages: MTLStages, visibilityOptions: MTL4VisibilityOptions = [ .device ])
```

## Parameters

- **afterQueueStages** — [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) mask that represents the stages of work to wait for. This argument applies to work corresponding to these stages you encode in prior command encoders, and not for the current encoder.
- **beforeStages** — [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) mask that represents the stages of work that wait. This argument applies to work you encode in the current command encoder.
- **visibilityOptions** — [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) of the barrier.

## Discussion

Encode a barrier that guarantees that any subsequent work you encode in the current command encoder that corresponds to the `beforeStages` stages doesn’t proceed until Metal completes all work prior to the current command encoder corresponding to the `afterQueueStages` stages, completes.

Metal can reorder the exact point where it applies the barrier, so encode the barrier as close to the command that consumes the resource as possible. Don’t use this method for synchronizing resource access within the same pass.

If you need to synchronize work within a pass that you encode with an instance of a subclass of [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder), use memory barriers instead. For subclasses of [MTL4CommandEncoder](https://developer.apple.com/documentation/metal/mtl4commandencoder), use encoder barriers.

You can specify `afterQueueStages` and `beforeStages` that contain [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) unrelated to the current command encoder.
