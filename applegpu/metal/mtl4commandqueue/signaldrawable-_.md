# signalDrawable(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/signaldrawable(_:)>

Schedules a signal operation on the command queue to indicate when rendering to a Metal drawable is complete.

## Declaration

```swift
func signalDrawable(_ drawable: any MTLDrawable)
```

## Parameters

- **drawable** — [MTLDrawable](https://developer.apple.com/documentation/metal/mtldrawable) instance to signal.

## Discussion

Signaling when rendering to a [MTLDrawable](https://developer.apple.com/documentation/metal/mtldrawable) instance is complete indicates that it’s safe to present it to the display.

You are responsible for calling this method after committing all command buffers that contain commands targeting this drawable, and before calling [present()](https://developer.apple.com/documentation/metal/mtldrawable/present()), [present(at:)](https://developer.apple.com/documentation/metal/mtldrawable/present(at:)), or [present(afterMinimumDuration:)](https://developer.apple.com/documentation/metal/mtldrawable/present(afterminimumduration:)).

> **Note:**
> This method doesn’t trigger the presentation of the drawable, and fails if you call it after any of the present methods, or if you call it multiple times.

Metal doesn’t guarantee that command buffers you commit to the command queue after calling this method execute before presentation.
