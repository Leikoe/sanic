# init(rawValue:)

*Initializer · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlgpufamily/init(rawvalue:)>

Creates a GPU family instance from a raw value.

## Declaration

```swift
init?(rawValue: Int)
```

## Parameters

- **rawValue** — An integer value that represents a GPU family.

## Discussion

You don’t need to call this initializer because it’s part of how Swift represents an enumeration from an Objective-C framework.

> **Tip:**
>  Use one of the [MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily) cases, such as [MTLGPUFamily.metal3](https://developer.apple.com/documentation/metal/mtlgpufamily/metal3), instead of this initializer.
