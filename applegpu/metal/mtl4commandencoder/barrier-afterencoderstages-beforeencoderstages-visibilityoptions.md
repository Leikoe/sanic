# barrier(afterEncoderStages:beforeEncoderStages:visibilityOptions:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterencoderstages:beforeencoderstages:visibilityoptions:)>

Encodes an intra-pass barrier.

## Declaration

```swift
func barrier(afterEncoderStages: MTLStages, beforeEncoderStages: MTLStages, visibilityOptions: MTL4VisibilityOptions = [ .device ])
```

## Parameters

- **afterEncoderStages** — [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) mask that represents the stages of work to wait for. This argument only applies to subsequent work you encode in the current command encoder.
- **beforeEncoderStages** — [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) mask that represents the stages of work that wait. This argument only applies to work you encode in the current command encoder prior to this barrier.
- **visibilityOptions** — [MTL4VisibilityOptions](https://developer.apple.com/documentation/metal/mtl4visibilityoptions) of the barrier, controlling cache flush behavior.

## Discussion

Encode a barrier that guarantees that any subsequent work you encode in the *current command encoder*, corresponding to `beforeEncoderStages`, doesn’t begin until all prior commands in this command encoder, corresponding to `afterEncoderStages`, completes.

When calling this method, it’s your responsibility to ensure parameters `afterEncoderStages` and `beforeEncoderStages` contain a combination of [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) for which this encoder can encode commands. For example, for a [MTL4ComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtl4computecommandencoder) instance, you can provide any combination of [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch), [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) and [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure).
