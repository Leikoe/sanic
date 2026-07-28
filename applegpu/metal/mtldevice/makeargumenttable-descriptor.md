# makeArgumentTable(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/makeargumenttable(descriptor:)>

Creates a new argument table from an argument table descriptor.

## Declaration

```swift
func makeArgumentTable(descriptor: MTL4ArgumentTableDescriptor) throws -> any MTL4ArgumentTable
```

## Parameters

- **descriptor** — A [MTL4ArgumentTableDescriptor](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor) instance that configures the [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) instance.

## Return Value

A [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) instance, or `nil` if the function failed.
