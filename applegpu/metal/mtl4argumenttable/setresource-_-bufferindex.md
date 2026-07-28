# setResource(_:bufferIndex:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4argumenttable/setresource(_:bufferindex:)>

Binds a resource to a buffer binding slot.

## Declaration

```swift
func setResource(_ resourceID: MTLResourceID, bufferIndex bindingIndex: Int)
```

## Parameters

- **resourceID** — The [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid) of the Metal resource to bind.
- **bindingIndex** — A valid binding index in the buffer binding range. It is an error for this value to match or exceed the value of property [maxBufferBindCount](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor/maxbufferbindcount) on the descriptor from which you created this argument table.
