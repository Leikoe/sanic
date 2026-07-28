# MTLRasterizationRateMapDescriptor

*Class · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor>

An object that you use to configure new rasterization rate maps.

## Declaration

```swift
class MTLRasterizationRateMapDescriptor
```

## Overview

To create a new rate map, first create an [MTLRasterizationRateMapDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor) instance and set its property values. Then, create a new rasterization rate-map by calling an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s
[makeRasterizationRateMap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makerasterizationratemap(descriptor:)) method.

When creating a rate map, Metal copies into it property values from the descriptor. You can reuse a descrptor by modifying its property values, which doesn’t affect the other rate-map instances that already exist.

## Topics

### Creating rate map descriptors
- [init(screenSize:label:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:label:)) — A convenience initializer that creates a rate map descriptor with a given size and identifier.
- [init(screenSize:layer:label:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:layer:label:)) — A convenience initializer that creates a rate map descriptor with a single rate layer.
- [init(screenSize:layers:label:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/init(screensize:layers:label:)) — A convenience initializer that creates a rate map descriptor with a set of layer descriptors.

### Identifying the rate map
- [label](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/label) — A string used to identify the rate map you create with the descriptor.

### Configuring the viewport size
- [screenSize](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/screensize) — The size of the viewport coordinate system, in logical pixels.

### Configuring the rate map layers
- [layerCount](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layercount) — The number of layers in the rate map.
- [layer(at:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layer(at:)) — Returns the layer description for a layer in the rate map.
- [setLayer(_:at:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/setlayer(_:at:)) — Sets a configuration for a layer rate map.
- [layers](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/layers) — The rasterization rates for one or more layers in the rate map.
- [MTLRasterizationRateLayerArray](https://developer.apple.com/documentation/metal/mtlrasterizationratelayerarray) — Descriptions for the rasterization rates to apply to the set of layers in a rate map.

## See also

### Rasterization settings
- [Rendering at different rasterization rates](https://developer.apple.com/documentation/metal/rendering-at-different-rasterization-rates) — Configure a rasterization rate map to vary rasterization rates depending on the amount of detail needed.
- [Creating a rasterization rate map](https://developer.apple.com/documentation/metal/creating-a-rasterization-rate-map) — Define the rasterization rates for each part of your render target.
- [Rendering with a rasterization rate map](https://developer.apple.com/documentation/metal/rendering-with-a-rasterization-rate-map) — Create offscreen textures to hold intermediate rasterized data.
- [Scaling variable rasterization rate content](https://developer.apple.com/documentation/metal/scaling-variable-rasterization-rate-content) — Use the rate map data to scale the content to fill your destination texture.
- [MTLRasterizationRateMap](https://developer.apple.com/documentation/metal/mtlrasterizationratemap) — A compiled read-only instance that determines how to apply variable rasterization rates when rendering.
- [MTLCoordinate2D](https://developer.apple.com/documentation/metal/mtlcoordinate2d) — A coordinate in the viewport.
- [MTLCoordinate2DMake(_:_:)](https://developer.apple.com/documentation/metal/mtlcoordinate2dmake(_:_:)) — Returns a new 2D point with the specified coordinates.
