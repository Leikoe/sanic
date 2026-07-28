# makeDepthStencilState(descriptor:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makedepthstencilstate(descriptor:)>

Creates a depth-stencil state instance.

## Declaration

```swift
func makeDepthStencilState(descriptor: MTLDepthStencilDescriptor) -> (any MTLDepthStencilState)?
```

## Parameters

- **descriptor** — An [MTLDepthStencilDescriptor](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor) instance.

## Return Value

A new [MTLDepthStencilState](https://developer.apple.com/documentation/metal/mtldepthstencilstate) instance if the method completed successfully; otherwise `nil`.
