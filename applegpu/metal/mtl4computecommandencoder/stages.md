# stages()

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/stages()>

Queries a bitmask representing the shader stages on which commands currently present in this command encoder operate.

## Declaration

```swift
func stages() -> MTLStages
```

## Return Value

A bitmask representing shader stages that commands currently present in this command encoder operate on.

## Discussion

Metal dynamically updates this property based on the commands you encode into the command encoder, for example, it sets the bit [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) if this encoder contains any commands that dispatch a compute kernel.

Similarly, it sets the bit [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) if this encoder contains any commands to copy or modify buffers, textures, or indirect command buffers.

Finally, Metal sets the bit [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) if this encoder contains any commands that build, copy, or refit acceleration structures.
