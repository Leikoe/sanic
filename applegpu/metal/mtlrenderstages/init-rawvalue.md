# init(rawValue:)

*Initializer · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderstages/init(rawvalue:)>

Creates a render stage from a raw value.

## Declaration

```swift
init(rawValue: UInt)
```

## Parameters

- **rawValue** — A bit field value of a render stage as an integer.

## Discussion

Use of the [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) type’s static properties, such as [mesh](https://developer.apple.com/documentation/metal/mtlrenderstages/mesh), [vertex](https://developer.apple.com/documentation/metal/mtlrenderstages/vertex), or [fragment](https://developer.apple.com/documentation/metal/mtlrenderstages/fragment) instead of creating a render stage instance yourself with this initializer.
