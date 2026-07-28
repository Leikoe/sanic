# waitForEvent(_:value:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/waitforevent(_:value:)>

Schedules an operation to wait for a GPU event of a specific value before continuing to execute any future GPU work.

## Declaration

```swift
func waitForEvent(_ event: any MTLEvent, value: UInt64)
```

## Parameters

- **event** — [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) to wait on.
- **value** — The specific value to wait for.
