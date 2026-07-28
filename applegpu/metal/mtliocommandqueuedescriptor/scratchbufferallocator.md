# scratchBufferAllocator

*Instance Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/scratchbufferallocator>

An optional memory allocator that you implement to manage the scratch memory that an input/output command queue requests.

## Declaration

```swift
var scratchBufferAllocator: (any MTLIOScratchBufferAllocator)? { get set }
```

## Discussion

Your app can manage an input/output command queue’s scratch memory by an implementing [MTLIOScratchBufferAllocator](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator) in one of your types, and assigning an instance of it to [scratchBufferAllocator](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/scratchbufferallocator). Otherwise, set to `nil` to instruct the input/output command queue to allocate and manage its own scratch buffers.

> **Note:**
>  An input/output command queue uses scratch buffers for memory-intensives tasks, including loading textures and decompressing asset files.
