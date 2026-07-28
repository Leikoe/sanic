# present()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldrawable/present()>

Presents the drawable onscreen as soon as possible.

## Declaration

```swift
func present()
```

## Discussion

When a command queue schedules a command buffer for execution, it tracks whether any commands in that command buffer need to render or write to the drawable object. When you call this method, the drawable presents its contents as soon as possible after all scheduled render or write requests for that drawable are complete.

> **Note:**
>  To avoid presenting a drawable before any work is scheduled, or to avoid holding on to a drawable longer than necessary, call a command buffer’s [present(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:)) method instead of this method. The [present(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:)) method is a convenience method that calls the drawable’s [present()](https://developer.apple.com/documentation/metal/mtldrawable/present()) method after the command queue schedules that command buffer for execution.

## See also

### Presenting the drawable
- [present(afterMinimumDuration:)](https://developer.apple.com/documentation/metal/mtldrawable/present(afterminimumduration:)) — Presents the drawable onscreen as soon as possible after a previous drawable is visible for the specified duration.
- [present(at:)](https://developer.apple.com/documentation/metal/mtldrawable/present(at:)) — Presents the drawable onscreen at a specific host time.
