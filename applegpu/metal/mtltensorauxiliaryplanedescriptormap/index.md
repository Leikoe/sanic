# MTLTensorAuxiliaryPlaneDescriptorMap

*Class · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptormap>

A map of auxiliary plane descriptors keyed by plane type.

## Declaration

```swift
class MTLTensorAuxiliaryPlaneDescriptorMap
```

## Overview

Use this collection to associate [MTLTensorPlaneType](https://developer.apple.com/documentation/metal/mtltensorplanetype) values with [MTLTensorAuxiliaryPlaneDescriptor](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptor) configurations, then attach it to a [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor) to create a multi-plane tensor.

## Topics

### Instance Methods
- [descriptor(for:)](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptormap/descriptor(for:)) — Returns the auxiliary plane descriptor for the given plane type, or `nil` if none has been set.
- [reset()](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptormap/reset()) — Empties the map of all its elements.
- [setDescriptor(_:for:)](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptormap/setdescriptor(_:for:)) — Sets the auxiliary plane descriptor for the given plane type.
