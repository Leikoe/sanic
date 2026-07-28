# MTLResourceUsage

*Structure · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourceusage>

Options that describe how a graphics or compute function uses an argument buffer’s resource.

## Declaration

```swift
struct MTLResourceUsage
```

## Overview

You can combine multiple [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) values with a bitwise OR (`|`) if the resource serves multiple purposes over its lifetime. You can enable options for certain resources that indicate whether the Metal driver needs to convert the resource to another format, such as whether it needs to decompress a color render target.

## Topics

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlresourceusage/init(rawvalue:)) — Creates a set of resource options from a raw value.

### Type Properties
- [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read) — An option that enables reading from the resource.
- [sample](https://developer.apple.com/documentation/metal/mtlresourceusage/sample) — An option that enables sampling from the resource.
- [write](https://developer.apple.com/documentation/metal/mtlresourceusage/write) — An option that enables writing to the resource.

## See also

### Common resource functionality
- [MTLGPUAddress](https://developer.apple.com/documentation/metal/mtlgpuaddress) — A 64-bit unsigned integer type appropriate for storing GPU addresses.
- [MTLAllocation](https://developer.apple.com/documentation/metal/mtlallocation) — A memory allocation from a Metal GPU device, such as a memory heap, texture, or data buffer.
- [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) — An allocation of memory accessible to a GPU.
- [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) — Optional arguments used to set the behavior of a resource.
- [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid)
