# makeSharedEvent(handle:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makesharedevent(handle:)>

Recreates a shared event from a handle.

## Declaration

```swift
func makeSharedEvent(handle sharedEventHandle: MTLSharedEventHandle) -> (any MTLSharedEvent)?
```

## Parameters

- **sharedEventHandle** — An [MTLSharedEventHandle](https://developer.apple.com/documentation/metal/mtlsharedeventhandle) instance from another GPU device or process.

## Return Value

A new [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) instance if the method completed successfully; otherwise nil.

## See also

### Creating fences and events
- [makeFence()](https://developer.apple.com/documentation/metal/mtldevice/makefence()) — Creates a new memory fence instance.
- [makeEvent()](https://developer.apple.com/documentation/metal/mtldevice/makeevent()) — Creates a new event instance that you can use to synchronize commands and resources within the same GPU device.
- [makeSharedEvent()](https://developer.apple.com/documentation/metal/mtldevice/makesharedevent()) — Creates a new shared event instance that you can use to synchronize commands and resources across different GPU devices.
