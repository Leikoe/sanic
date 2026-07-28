# barrier(afterStages:beforeQueueStages:visibilityOptions:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterstages:beforequeuestages:visibilityoptions:)>

Encodes a producer barrier on work committed to the same command queue.

## Declaration

```swift
func barrier(afterStages: MTLStages, beforeQueueStages: MTLStages, visibilityOptions: MTL4VisibilityOptions = [ .device ])
```

## Parameters

- **afterStages** — [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) mask that represents the stages of work to wait for. This argument applies to work corresponding to these stages you encode in the current command encoder prior to this barrier command.
- **beforeQueueStages** — [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) mask that represents the stages of work that need to wait. This argument applies to subsequent encoders and not to work in the current command encoder.
- **visibilityOptions** — [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) of the barrier, controlling cache flush behavior.

## Discussion

This method encodes a barrier that guarantees that any work you encode using *subsequent command encoders*, corresponding to `beforeQueueStages`, don’t begin until all commands you previously encode in the current encoder (and prior encoders), corresponding to `afterStages`, complete.

When calling this method, you can pass any [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) to parameters `afterStages` and `beforeQueueStages`, even stages that don’t relate to the current or prior command encoders.
