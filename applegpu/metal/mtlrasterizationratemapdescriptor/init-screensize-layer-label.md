# init(screenSize:layer:label:)

*Initializer · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:layer:label:)>

A convenience initializer that creates a rate map descriptor with a single rate layer.

## Declaration

```swift
convenience init(screenSize: MTLSize, layer: MTLRasterizationRateLayerDescriptor, label: String? = nil)
```

## Parameters

- **screenSize** — The logical size, in pixels, of the viewport coordinate system.
- **layer** — A descriptor for the rate layer to create.
- **label** — A string that identifies the resulting rate map.

## Return Value

A descriptor object whose [screenSize](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/screensize) and [label](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/label) properties are set to the provided values. Layer `0` in the rate map is set to the provided layer descriptor.

## See also

### Creating rate map descriptors
- [init(screenSize:label:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:label:)) — A convenience initializer that creates a rate map descriptor with a given size and identifier.
- [init(screenSize:layers:label:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:layers:label:)) — A convenience initializer that creates a rate map descriptor with a set of layer descriptors.
