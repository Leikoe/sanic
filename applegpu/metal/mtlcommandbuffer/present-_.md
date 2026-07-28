# present(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:)>

Presents a drawable as early as possible.

## Declaration

```swift
func present(_ drawable: any MTLDrawable)
```

## Parameters

- **drawable** — An [MTLDrawable](https://developer.apple.com/documentation/metal/mtldrawable) instance that contains a texture the system can show on a display.

## Discussion

This convenience method calls the drawable’s [present()](https://developer.apple.com/documentation/metal/mtldrawable/present()) method after the command queue schedules the command buffer for execution. The command buffer does this by adding a completion handler by calling its own [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) method for you.

> **Important:**
>  You can only call this method before calling the command buffer’s [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method.

## See also

### Presenting a drawable
- [present(_:atTime:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:attime:)) — Presents a drawable at a specific time.
- [present(_:afterMinimumDuration:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:afterminimumduration:)) — Presents a drawable after the system presents the previous drawable for an amount of time.
