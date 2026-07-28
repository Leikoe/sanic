# feedbackQueue

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueuedescriptor/feedbackqueue>

Assigns a dispatch queue to which Metal submits feedback notification blocks.

## Declaration

```swift
unowned(unsafe) var feedbackQueue: dispatch_queue_t? { get set }
```

## Discussion

When you assign a dispatch queue via this method, Metal requires that the queue parameter you provide is a serial queue.

If you set the value of property to `nil`, the default, Metal allocates an internal dispatch queue to service feedback notifications.
