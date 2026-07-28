# MTLTensorAuxiliaryPlaneDescriptor

*Class · iOS 27.0, iPadOS 27.0, Mac Catalyst 27.0, macOS 27.0, tvOS 27.0, visionOS 27.0*

<https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptor>

A configuration for an auxiliary plane in a multi-plane tensor.

## Declaration

```swift
class MTLTensorAuxiliaryPlaneDescriptor
```

## Overview

Use this descriptor to configure an auxiliary plane’s data type and block factors before attaching it to a [MTLTensorDescriptor](https://developer.apple.com/documentation/metal/mtltensordescriptor).

## Topics

### Instance Properties
- [blockFactors](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptor/blockfactors) — An extents instance that represents the number of data plane elements which correspond to one element in a plane you create with this descriptor.
- [dataType](https://developer.apple.com/documentation/metal/mtltensorauxiliaryplanedescriptor/datatype) — The data format of all elements in the plane.
