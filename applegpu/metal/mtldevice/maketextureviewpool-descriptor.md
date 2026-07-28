# makeTextureViewPool(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/maketextureviewpool(descriptor:)>

Creates a new texture view pool from a resource view pool descriptor.

## Declaration

```swift
func makeTextureViewPool(descriptor: MTLResourceViewPoolDescriptor) throws -> any MTLTextureViewPool
```

## Parameters

- **descriptor** — A [MTLResourceViewPoolDescriptor](https://developer.apple.com/documentation/metal/mtlresourceviewpooldescriptor) instance that configures the [MTLTextureViewPool](https://developer.apple.com/documentation/metal/mtltextureviewpool) instance.

## Return Value

A [MTLTextureViewPool](https://developer.apple.com/documentation/metal/mtltextureviewpool) instance, or `nil` if the function failed.
