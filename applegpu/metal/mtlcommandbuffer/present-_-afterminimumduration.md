# present(_:afterMinimumDuration:)

*Instance Method · iOS 10.3, iPadOS 10.3, Mac Catalyst 13.4, macOS 10.15.4, tvOS 10.2, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:afterminimumduration:)>

Presents a drawable after the system presents the previous drawable for an amount of time.

## Declaration

```swift
func present(_ drawable: any MTLDrawable, afterMinimumDuration duration: CFTimeInterval)
```

## Parameters

- **drawable** — An [MTLDrawable](https://developer.apple.com/documentation/metal/mtldrawable) instance that contains a texture the system can show on a display.
- **duration** — The shortest display time you want the system to give to the previous drawable before presenting this one.

## Discussion

This convenience method calls the drawable’s [present(afterMinimumDuration:)](https://developer.apple.com/documentation/metal/mtldrawable/present(afterminimumduration:)) method after the command queue schedules the command buffer for execution. The command buffer does this by adding a completion handler by calling its own [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) method for you.

> **Important:**
>  You can only call this method before calling the command buffer’s [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method.

## See also

### Presenting a drawable
- [present(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:)) — Presents a drawable as early as possible.
- [present(_:atTime:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:attime:)) — Presents a drawable at a specific time.
