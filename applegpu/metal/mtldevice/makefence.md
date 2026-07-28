# makeFence()

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makefence()>

Creates a new memory fence instance.

## Declaration

```swift
func makeFence() -> (any MTLFence)?
```

## See also

### Creating fences and events
- [makeEvent()](https://developer.apple.com/documentation/metal/mtldevice/makeevent()) — Creates a new event instance that you can use to synchronize commands and resources within the same GPU device.
- [makeSharedEvent()](https://developer.apple.com/documentation/metal/mtldevice/makesharedevent()) — Creates a new shared event instance that you can use to synchronize commands and resources across different GPU devices.
- [makeSharedEvent(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedevent(handle:)) — Recreates a shared event from a handle.
