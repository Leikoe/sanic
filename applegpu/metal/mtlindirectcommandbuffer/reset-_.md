# reset(_:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 12.0, macOS 10.14, tvOS 12.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/reset(_:)>

Resets a range of commands to their default state.

## Declaration

```swift
func reset(_ range: Range<Int>)
```

## Parameters

- **range** — The range of commands to reset. The range needs to fit inside the indirect command buffer’s extents.
