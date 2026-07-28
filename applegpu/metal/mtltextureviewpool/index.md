# MTLTextureViewPool

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltextureviewpool>

A pool of lightweight texture views.

## Declaration

```swift
protocol MTLTextureViewPool : MTLResourceViewPool
```

## Overview

Use texture view pools to create lightweight texture view objects of [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) and [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instances.

## Topics

### Instance Methods
- [setTextureView(buffer:descriptor:offset:bytesPerRow:index:)](https://developer.apple.com/documentation/metal/mtltextureviewpool/settextureview(buffer:descriptor:offset:bytesperrow:index:)) — Creates a new lightweight texture view of a buffer.
- [setTextureView(texture:descriptor:index:)](https://developer.apple.com/documentation/metal/mtltextureviewpool/settextureview(texture:descriptor:index:)) — Creates a new lightweight texture view.
- [setTextureView(texture:index:)](https://developer.apple.com/documentation/metal/mtltextureviewpool/settextureview(texture:index:)) — Copies a default texture view to a slot in this texture view pool at an index provided.

## See also

### View pools
- [MTLResourceViewPool](https://developer.apple.com/documentation/metal/mtlresourceviewpool) — Contains views over resources of a specific type, and allows you to manage those views.
- [MTLResourceViewPoolDescriptor](https://developer.apple.com/documentation/metal/mtlresourceviewpooldescriptor) — Provides parameters for creating a resource view pool.
- [MTLTextureViewDescriptor](https://developer.apple.com/documentation/metal/mtltextureviewdescriptor)
