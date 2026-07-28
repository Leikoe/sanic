# makeIndirectCommandBuffer(descriptor:maxCommandCount:options:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makeindirectcommandbuffer(descriptor:maxcommandcount:options:)>

Creates an indirect command buffer instance.

## Declaration

```swift
func makeIndirectCommandBuffer(descriptor: MTLIndirectCommandBufferDescriptor, maxCommandCount maxCount: Int, options: MTLResourceOptions = []) -> (any MTLIndirectCommandBuffer)?
```

## Parameters

- **descriptor** — An [MTLIndirectCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor) instance.
- **maxCount** — The largest number of commands you can store in the buffer.
- **options** — An [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) instance.

## Return Value

A new [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance if the method completed successfully; otherwise `nil`.
