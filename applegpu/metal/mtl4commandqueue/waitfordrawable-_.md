# waitForDrawable(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/waitfordrawable(_:)>

Schedules a wait operation on the command queue to ensure the display is no longer using a specific Metal drawable.

## Declaration

```swift
func waitForDrawable(_ drawable: any MTLDrawable)
```

## Parameters

- **drawable** — [MTLDrawable](https://developer.apple.com/documentation/metal/mtldrawable) instance to signal.

## Discussion

Use this method to ensure the display is no longer using a [MTLDrawable](https://developer.apple.com/documentation/metal/mtldrawable) instance before executing any subsequent commands.

This method returns immediately and doesn’t perform any synchronization on the current thread. You are responsible for calling this method before committing any command buffers containing commands that target this drawable.

Call this method multiple times if you commit your command buffers to multiple command queues.
