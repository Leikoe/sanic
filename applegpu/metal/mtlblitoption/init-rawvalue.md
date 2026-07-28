# init(rawValue:)

*Initializer · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitoption/init(rawvalue:)>

Creates a blit option from a raw value.

## Declaration

```swift
init(rawValue: UInt)
```

## Parameters

- **rawValue** — The bitwise value of a blit option as an integer.

## Discussion

Use one of the [MTLBlitOption](https://developer.apple.com/documentation/metal/mtlblitoption) type’s static properties, such as [depthFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/depthfromdepthstencil), [stencilFromDepthStencil](https://developer.apple.com/documentation/metal/mtlblitoption/stencilfromdepthstencil), and [rowLinearPVRTC](https://developer.apple.com/documentation/metal/mtlblitoption/rowlinearpvrtc) instead of creating a blit option yourself with this initializer.
