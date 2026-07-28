# Creating a rasterization rate map

*Article*

<https://developer.apple.com/documentation/metal/creating-a-rasterization-rate-map>

Define the rasterization rates for each part of your render target.

## Overview

A rasterization rate map specifies the size of the final render target in logical viewport coordinates and rasterization rates within that area. You partition the render target into different horizontal and vertical zones and provide rasterization rates for each of these zones. From this configuration data, the rate map calculates the sizes for your intermediate texture targets and provides mapping functions between logical viewport coordinates and physical pixel coordinates.

To create a rate map, you create and configure a rate map descriptor for it and ask the device instance to create it. You then keep the rate map around for as long as you need it for your render targets.

For example, if you’re rendering for display to the screen, set the [screenSize](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor/screensize) property of the rate map descriptor to the [drawableSize](https://developer.apple.com/documentation/QuartzCore/CAMetalLayer/drawableSize) property of the destination [CAMetalLayer](https://developer.apple.com/documentation/QuartzCore/CAMetalLayer) instance.

```swift
let descriptor = MTLRasterizationRateMapDescriptor()
descriptor.label = "My rate map"

let layerWidth = Int(metalLayer.drawableSize.width)
let layerHeight = Int(metalLayer.drawableSize.height)
descriptor.screenSize = MTLSizeMake(layerWidth, layerHeight, 0)
```

```objective-c
MTLRasterizationRateMapDescriptor *descriptor = [[MTLRasterizationRateMapDescriptor alloc] init];
descriptor.label = @"My rate map";
descriptor.screenSize = destinationMetalLayer.drawableSize;
```

### Create a layer rate descriptor

To specify rasterization rates, create a layer rate descriptor with the rates you want to apply to each layer in the render target. If you aren’t using layered rendering, create a single layer rate descriptor. Otherwise, you can provide different rasterization rates for each layer. (For more information about layered rendering, see [Rendering to multiple texture slices in a draw command](https://developer.apple.com/documentation/metal/rendering-to-multiple-texture-slices-in-a-draw-command)).

Decide how many horizontal and vertical zones you need for each layer. The number of zones should be factors of the width and height of the screen size you specified, and you should choose as many zones as you need for your specific use case. If you need to provide a more precise grid, use more zones.

To specify the grid layout, create an [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance to hold the number of zones, and then create the layer descriptor:

```swift
let zoneCounts = MTLSizeMake(8, 4, 1)
let layerDescriptor = MTLRasterizationRateLayerDescriptor(sampleCount: zoneCounts)
```

```objective-c
MTLSize zoneCounts = MTLSizeMake(8, 4, 1);
MTLRasterizationRateLayerDescriptor *layerDescriptor = [[MTLRasterizationRateLayerDescriptor alloc] initWithSampleCount:zoneCounts];
```

### Specify the rates for each zone

After creating the layer descriptor, specify rates for the rows and columns of the rate map. You determine the horizontal rate for a cell by specifying the rate for its column, and its vertical rate by specifying the rate for its row.

The rate is a floating-point number from `0.0` to `1.0`, where `1.0` means that the hardware should rasterize that zone at the full rate. The following example specifies a full rate for each zone, the default Metal behavior:

```swift
for row in 0..<zoneCounts.height {
    layerDescriptor.vertical[row] = 1.0
}
for column in 0..<zoneCounts.width {
    layerDescriptor.horizontal[column] = 1.0
}
```

```objective-c
for (int row = 0; row < zoneCounts.height; row++)
{
    layerDescriptor.verticalSampleStorage[row] = 1.0;    
}
for (int column = 0; column < zoneCounts.width; column++)
{
    layerDescriptor.horizontalSampleStorage[column] = 1.0;
}
```

If you specify a value lower than `1.0`, the GPU rasterizes fewer pixels for that zone. For example, the following example code samples the edge zones at half the normal rate:

```swift
layerDescriptor.horizontal[0] = 0.5
layerDescriptor.horizontal[7] = 0.5
layerDescriptor.vertical[0] = 0.5
layerDescriptor.vertical[3] = 0.5
```

```objective-c
layerDescriptor.horizontalSampleStorage[0] = 0.5;
layerDescriptor.horizontalSampleStorage[7] = 0.5;
layerDescriptor.verticalSampleStorage[0] = 0.5;
layerDescriptor.verticalSampleStorage[3] = 0.5;
```

Metal guarantees that the actual rasterization rates are at least as high as the rates you specified. However, when you create the rate map, the device instance may split it into more detailed cells or choose higher rates for specific cells if the GPU requires it.

### Add the layer descriptor to the rate map descriptor

After you configure the layer descriptor, attach it to the rate map descriptor. When you’ve added all of the layer descriptors, create the rate map:

```swift
descriptor.setLayer(layerDescriptor, at: 0)
let rateMap = device.makeRasterizationRateMap(descriptor: descriptor)
```

```objective-c
[descriptor setLayer:layerDescriptor atIndex:0];
id<MTLRasterizationRateMap> rateMap = [_device newRasterizationRateMapWithDescriptor: descriptor];
```

## See also

### Rasterization settings
- [Rendering at different rasterization rates](https://developer.apple.com/documentation/metal/rendering-at-different-rasterization-rates) — Configure a rasterization rate map to vary rasterization rates depending on the amount of detail needed.
- [Rendering with a rasterization rate map](https://developer.apple.com/documentation/metal/rendering-with-a-rasterization-rate-map) — Create offscreen textures to hold intermediate rasterized data.
- [Scaling variable rasterization rate content](https://developer.apple.com/documentation/metal/scaling-variable-rasterization-rate-content) — Use the rate map data to scale the content to fill your destination texture.
- [MTLRasterizationRateMapDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor) — An object that you use to configure new rasterization rate maps.
- [MTLRasterizationRateMap](https://developer.apple.com/documentation/metal/mtlrasterizationratemap) — A compiled read-only instance that determines how to apply variable rasterization rates when rendering.
- [MTLCoordinate2D](https://developer.apple.com/documentation/metal/mtlcoordinate2d) — A coordinate in the viewport.
- [MTLCoordinate2DMake(_:_:)](https://developer.apple.com/documentation/metal/mtlcoordinate2dmake(_:_:)) — Returns a new 2D point with the specified coordinates.
