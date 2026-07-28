# MTLRasterizationRateMap

*Protocol · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.4, macOS 10.15.4, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrasterizationratemap>

A compiled read-only instance that determines how to apply variable rasterization rates when rendering.

## Declaration

```swift
protocol MTLRasterizationRateMap : NSObjectProtocol, Sendable
```

## Overview

Use a rasterization rate map to reduce rendering quality in less-important or less-sampled regions of the render target, such as areas affected by blur effects or a far-away cascade of a shadow map.

By default, a render pass doesn’t have a rasterization rate map, and the viewport coordinate system maps exactly to physical pixels in the targeted textures. If you apply a rasterization rate map to a render pass, the viewport coordinate system becomes a logical coordinate system, and the rate map describes how to map logical coordinates to physical pixels in the render pass’s targets. You can specify different rasterization rates in different regions of the logical coordinate system. When you do, those logical units map to fewer physical pixels, which means you can use smaller render targets and render fewer pixels, saving both memory and processing time. For more information, see [Rendering at different rasterization rates](https://developer.apple.com/documentation/metal/rendering-at-different-rasterization-rates).

Don’t implement this protocol yourself; instead, create an [MTLRasterizationRateMapDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor) instance, configure it, and then call the [makeRasterizationRateMap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makerasterizationratemap(descriptor:)) on a device instance.

To apply a rasterization rate map to a render pass, set the render pass descriptor’s [rasterizationRateMap](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/rasterizationratemap) property.

### Configuring the rate map

A rasterization rate map specifies the size of the viewport coordinate space in logical units and one or more *layer maps*. A layer map partitions the viewport coordinate space into a 2D grid of cells and defines the rasterization rate for each cell. If you aren’t using layered rendering, provide a single layer map; otherwise, provide one layer map for each layer. For more information about layered rendering, see [Rendering to multiple texture slices in a draw command](https://developer.apple.com/documentation/metal/rendering-to-multiple-texture-slices-in-a-draw-command).

You can query the physical size requirements for each layer in the render pass by calling the [physicalSize(layer:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalsize(layer:)) method. Your render targets need to be at least this large.

## Topics

### Identifying the rate map
- [device](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/device) — The device object that created the rate map.
- [label](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/label) — A string that identifies the rate map.

### Inspecting geometric and rendering properties
- [layerCount](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/layercount) — The number of layers in the rate map.
- [screenSize](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/screensize) — The logical size, in pixels, of the viewport coordinate system.
- [physicalSize(layer:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalsize(layer:)) — Returns the dimensions, in pixels, of the area in the render target affected by the rasterization rate map.
- [physicalGranularity](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalgranularity) — The granularity, in physical pixels, at which the rasterization rate varies.

### Converting between viewport and physical coordinates
- [physicalCoordinates(screenCoordinates:layer:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalcoordinates(screencoordinates:layer:)) — Converts a point in logical viewport coordinates to the corresponding physical coordinates in a render layer.
- [screenCoordinates(physicalCoordinates:layer:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/screencoordinates(physicalcoordinates:layer:)) — Converts a point in physical coordinates inside a layer to its corresponding logical viewport coordinates.

### Obtaining coordinate transformation data
- [parameterDataSizeAndAlign](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/parameterdatasizeandalign) — The size and alignment requirements to contain the coordinate transformation information in this rate map.
- [copyParameterData(buffer:offset:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/copyparameterdata(buffer:offset:)) — Copies the parameter data into the provided buffer.

## See also

### Rasterization settings
- [Rendering at different rasterization rates](https://developer.apple.com/documentation/metal/rendering-at-different-rasterization-rates) — Configure a rasterization rate map to vary rasterization rates depending on the amount of detail needed.
- [Creating a rasterization rate map](https://developer.apple.com/documentation/metal/creating-a-rasterization-rate-map) — Define the rasterization rates for each part of your render target.
- [Rendering with a rasterization rate map](https://developer.apple.com/documentation/metal/rendering-with-a-rasterization-rate-map) — Create offscreen textures to hold intermediate rasterized data.
- [Scaling variable rasterization rate content](https://developer.apple.com/documentation/metal/scaling-variable-rasterization-rate-content) — Use the rate map data to scale the content to fill your destination texture.
- [MTLRasterizationRateMapDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor) — An object that you use to configure new rasterization rate maps.
- [MTLCoordinate2D](https://developer.apple.com/documentation/metal/mtlcoordinate2d) — A coordinate in the viewport.
- [MTLCoordinate2DMake(_:_:)](https://developer.apple.com/documentation/metal/mtlcoordinate2dmake(_:_:)) — Returns a new 2D point with the specified coordinates.
