# subscript(_:)

*Instance Subscript · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray/subscript(_:)>

Returns the state of the specified buffer layout.

## Declaration

```swift
subscript(index: Int) -> MTLBufferLayoutDescriptor! { get set }
```

## Parameters

- **index** — A specified index in the array of buffer layouts.

## Return Value

The buffer layout descriptor for the buffer bound to the given attribute table index.
