# signalEvent(_:value:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/signalevent(_:value:)>

Schedules an operation to signal a GPU event with a specific value after all GPU work prior to this point is complete.

## Declaration

```swift
func signalEvent(_ event: any MTLEvent, value: UInt64)
```

## Parameters

- **event** — [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) to signal.
- **value** — The value to signal the [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) with.
