# notify(_:atValue:block:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsharedevent/notify(_:atvalue:block:)>

Schedules a notification handler to be called after the shareable event’s signal value equals or exceeds a given value.

## Declaration

```swift
func notify(_ listener: MTLSharedEventListener, atValue value: UInt64, block: @escaping MTLSharedEventNotificationBlock)
```

## Parameters

- **listener** — The listener object used to dispatch the notification.
- **value** — The minimum value that needs to be signaled before the notification handler is called.
- **block** — The notification handler to call.

## See also

### Synchronizing a shareable event
- [signaledValue](https://developer.apple.com/documentation/metal/mtlsharedevent/signaledvalue) — The current signal value for the shareable event.
