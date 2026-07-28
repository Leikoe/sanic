# init(screenSize:label:)

*Initializer · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:label:)>

A convenience initializer that creates a rate map descriptor with a given size and identifier.

## Declaration

```swift
convenience init(screenSize: MTLSize, label: String? = nil)
```

## Parameters

- **screenSize** — The logical size, in pixels, of the viewport coordinate system.
- **label** — A string that identifies the resulting rate map.

## Return Value

A descriptor object whose [screenSize](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/screensize) and [label](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/label) properties are set to the provided values. You need to add at least one layer rate map to the descriptor.

## See also

### Creating rate map descriptors
- [init(screenSize:layer:label:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:layer:label:)) — A convenience initializer that creates a rate map descriptor with a single rate layer.
- [init(screenSize:layers:label:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:layers:label:)) — A convenience initializer that creates a rate map descriptor with a set of layer descriptors.
