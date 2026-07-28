# makeCommandAllocator(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/makecommandallocator(descriptor:)>

Creates a new command allocator from a command allocator descriptor.

## Declaration

```swift
func makeCommandAllocator(descriptor: MTL4CommandAllocatorDescriptor) throws -> any MTL4CommandAllocator
```

## Parameters

- **descriptor** — A [MTL4CommandAllocatorDescriptor](https://developer.apple.com/documentation/metal/mtl4commandallocatordescriptor) instance that configures the [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) instance.

## Return Value

A [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) instance, or `nil` if the function failed.
