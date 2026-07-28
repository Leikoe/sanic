# addFeedbackHandler(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commitoptions/addfeedbackhandler(_:)>

Registers a commit feedback handler that Metal calls with feedback data when available.

## Declaration

```swift
func addFeedbackHandler(_ block: @escaping MTL4CommitFeedbackHandler)
```

## Parameters

- **block** — [MTL4CommitFeedbackHandler](https://developer.apple.com/documentation/metal/mtl4commitfeedbackhandler) that Metal invokes.
