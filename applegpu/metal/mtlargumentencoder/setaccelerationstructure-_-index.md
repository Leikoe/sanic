# setAccelerationStructure(_:index:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setaccelerationstructure(_:index:)>

Encodes a reference to an acceleration structure into the argument buffer.

## Declaration

```swift
func setAccelerationStructure(_ accelerationStructure: (any MTLAccelerationStructure)?, index: Int)
```

## Parameters

- **accelerationStructure** — An acceleration structure the method encodes.
- **index** — The index of an acceleration structure within the argument buffer. The value corresponds to either the index ID of a declaration in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of an [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instance.
