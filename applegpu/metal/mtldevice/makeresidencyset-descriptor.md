# makeResidencySet(descriptor:)

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtldevice/makeresidencyset(descriptor:)>

Creates a residency set, which can move resources in and out of memory residency.

## Declaration

```swift
func makeResidencySet(descriptor desc: MTLResidencySetDescriptor) throws -> any MTLResidencySet
```

## Parameters

- **desc** — A descriptor instance that configures the residency set the method creates.

## Return Value

A new [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## Discussion

Create an [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) by creating and configuring an [MTLResidencySetDescriptor](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor) instance and pass it to this method.

See [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) for more information.
