# makeScratchBuffer(minimumSize:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator/makescratchbuffer(minimumsize:)>

Creates a scratch memory buffer for an input/output command queue.

## Declaration

```swift
func makeScratchBuffer(minimumSize: Int) -> (any MTLIOScratchBuffer)?
```

## Parameters

- **minimumSize** — The number of bytes the input/output command buffer needs to successfully run a command buffer.

## Return Value

An [MTLIOScratchBuffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer) instance that your app implements or `nil`.

## Discussion

Your app can reduce additional callbacks from the framework by providing additional memory above `minimumSize`. If your implementation returns `nil`, the input/output command queue cancels the [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) instance that needs the scratch buffer memory.
