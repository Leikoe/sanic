# makeLogState(descriptor:)

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtldevice/makelogstate(descriptor:)>

Creates a shader log state with the provided configuration.

## Declaration

```swift
func makeLogState(descriptor: MTLLogStateDescriptor) throws -> any MTLLogState
```

## Parameters

- **descriptor** — The configuration for the new shader log state.

## Return Value

A new [MTLLogState](https://developer.apple.com/documentation/metal/mtllogstate) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.
