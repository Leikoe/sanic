# insertDebugCaptureBoundary()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandqueue/insertdebugcaptureboundary()>

Informs Xcode about when GPU Frame Capture starts and stops.

## Declaration

```swift
func insertDebugCaptureBoundary()
```

## Discussion

You can explicitly define the boundary between two GPU captures by calling this method, which overrides the default behavior in Xcode when you caputre a GPU frame. If your app doesn’t call the method, Xcode adds a frame boundary each time your app calls the [present(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:)) or [present(_:atTime:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:attime:)) methods.

For example, an app with a single drawable may not need this method because the default behavior’s implicit frame boundaries are appropriate for that scenario.

![image](https://docs-assets.developer.apple.com/published/bf7923f90777bbaa9eca991e7c32cf48/insertDebugCaptureBoundary-1%402x.png)

However, you may want to create explicit frame boundaries for apps with multiple drawables that produce frames at different rates.

![image](https://docs-assets.developer.apple.com/published/d172a062a3fe167f870c98379e2633ee/insertDebugCaptureBoundary-2%402x.png)

In this example scenario, the app uses three drawables, each of which presents their frames at different rates or times. The developer can use this method to add arbitrary boundaries that create two captures. The first capture contains the first two frames from Drawable A, the first frame from Drawable B, and the first frame from Drawable C. The second capture contains the third and fourth frames from Drawable A, the second frame from Drawable B, and the second and third frames from Drawable C.

> **Warning:**
>  Don’t call this method from within the completion handler you pass to [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) because it can trigger a deadlock when you capture a GPU frame.
