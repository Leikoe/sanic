# MTLResidencySetDescriptor

*Class · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor>

A configuration that customizes the behavior for a residency set.

## Declaration

```swift
class MTLResidencySetDescriptor
```

## Overview

Make an [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) by creating and configuring an [MTLResidencySetDescriptor](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor) instance and pass it to the [makeResidencySet(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeresidencyset(descriptor:)) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

See [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) for more information.

## Topics

### Configuring the residency set
- [label](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor/label) — An optional name that can help you identify a residency set you create with the descriptor.
- [initialCapacity](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor/initialcapacity) — The number of allocations a new residency set can store without reallocating memory.

## See also

### Residency sets
- [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) — Organize your resources into groups and influence when they become accessible to the GPU.
- [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) — A collection of resource allocations that can move in and out of resident memory.
