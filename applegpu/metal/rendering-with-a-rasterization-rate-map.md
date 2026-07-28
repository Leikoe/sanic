# Rendering with a rasterization rate map

*Article*

<https://developer.apple.com/documentation/metal/rendering-with-a-rasterization-rate-map>

Create offscreen textures to hold intermediate rasterized data.

## Overview

You don’t render data directly to the final render destination using variable rasterization rates. Instead, you allocate textures to use as intermediate targets. You get the sizes for these textures from the rate map. For example, if you specified a rate of `0.5` everywhere, the intermediate texture is half the height and width of the final render target.

The [MTLRasterizationRateMap](https://developer.apple.com/documentation/metal/mtlrasterizationratemap) protocol [screenSize](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/screensize) property gives the dimensions of the final render target. Call the [physicalSize(layer:)](https://developer.apple.com/documentation/metal/mtlrasterizationratemap/physicalsize(layer:)): method on the rate map object to get the size for each layer. Then allocate the textures you need. For example, the following code gets the physical size and allocates a color texture and a depth texture of that size:

```swift
// Get the size of each layer from the rate map object.
let textureSize = rateMap.physicalSize(layer: 0)

// Create a texture descriptor.
let colorTextureDescriptor = MTLTextureDescriptor()

// Configure the width and height from the rate map.
colorTextureDescriptor.width = textureSize.width
colorTextureDescriptor.height = textureSize.height

colorTextureDescriptor.pixelFormat = .bgra8Unorm_srgb

// Disable mipmapping for the texture(s).
colorTextureDescriptor.mipmapLevelCount = 1

// Configure the texture(s) as render target(s) that shaders can read.
colorTextureDescriptor.usage = [.renderTarget, .shaderRead]

// Create the color texture from the descriptor.
let colorTexture = device.makeTexture(descriptor: colorTextureDescriptor)

// Create a depth texture descriptor of the same size.
let depthTextureDescriptor = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .depth32Float,
                                width: textureSize.width, height: textureSize.height, mipmapped: false)
depthTextureDescriptor.usage = .renderTarget

// Create the depth texture from the descriptor.
let depthTexture = device.makeTexture(descriptor: depthTextureDescriptor)
```

```objective-c
// Get the size of each layer from the rate map object.
MTLSize textureSize = [_rateMap physicalSizeForLayer:0];

// Create a texture descriptor.
MTLTextureDescriptor *colorTextureDescriptor = [MTLTextureDescriptor
        texture2DDescriptorWithPixelFormat:MTLPixelFormatBGRA8Unorm_sRGB
        width:textureSize.width
        height:textureSize.height
        mipmapped:NO];
colorTextureDescriptor.usage = MTLTextureUsageShaderRead | MTLTextureUsageRenderTarget;

// Create the color texture from the descriptor.
id<MTLTexture> colorTexture = [_device newTextureWithDescriptor:colorTextureDescriptor];

// Create a depth texture descriptor of the same size.
MTLTextureDescriptor *depthTextureDescriptor = [MTLTextureDescriptor
        texture2DDescriptorWithPixelFormat:MTLPixelFormatDepth32Float_Stencil8
        width:textureSize.width
        height:textureSize.height mipmapped:NO];
depthTextureDescriptor.usage = MTLTextureUsageRenderTarget;

// Create the depth texture from the descriptor.
id<MTLTexture> depthTexture = [_device newTextureWithDescriptor:depthTextureDescriptor];
```

To use variable rasterization rates, create a render pass, specifying the intermediate textures as the render targets. Set the [rasterizationRateMap](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/rasterizationratemap) property to the rate map you created, as shown below:

```swift
let firstPassDescriptor = MTLRenderPassDescriptor()

firstPassDescriptor.rasterizationRateMap = rateMap

firstPassDescriptor.colorAttachments[0].texture = colorTexture
firstPassDescriptor.colorAttachments[0].loadAction = MTLLoadAction.clear
firstPassDescriptor.colorAttachments[0].storeAction = MTLStoreAction.store

firstPassDescriptor.depthAttachment.texture = depthTexture
firstPassDescriptor.depthAttachment.loadAction = MTLLoadAction.clear
firstPassDescriptor.depthAttachment.storeAction = MTLStoreAction.dontCare

firstPassDescriptor.stencilAttachment.texture = depthTexture
firstPassDescriptor.stencilAttachment.loadAction = MTLLoadAction.clear
firstPassDescriptor.stencilAttachment.storeAction = MTLStoreAction.dontCare
```

```objective-c
MTLRenderPassDescriptor* firstPassDescriptor = [[MTLRenderPassDescriptor alloc] init];

firstPassDescriptor.rasterizationRateMap = _rateMap;

firstPassDescriptor.colorAttachments[0].texture = colorTexture;
firstPassDescriptor.colorAttachments[0].loadAction = MTLLoadActionClear;
firstPassDescriptor.colorAttachments[0].storeAction = MTLStoreActionStore;

firstPassDescriptor.depthAttachment.texture = depthTexture;
firstPassDescriptor.depthAttachment.loadAction = MTLLoadActionClear;
firstPassDescriptor.depthAttachment.storeAction = MTLStoreActionDontCare;
firstPassDescriptor.stencilAttachment.texture = depthTexture;
firstPassDescriptor.stencilAttachment.loadAction = MTLLoadActionClear;
firstPassDescriptor.stencilAttachment.storeAction = MTLStoreActionDontCare;
```

After rendering with a rasterization rate map, you need to scale the intermediate textures to fill your destination texture. To convert between screen space and physical fragment space in your shader, bind the rate map data buffer to the shader with type `rasterization_rate_map_data`, then construct `rasterization_rate_map_decoder` with the buffer data. For more details, see the “Variable Rasterization Rate” section of the [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf) and [Scaling variable rasterization rate content](https://developer.apple.com/documentation/metal/scaling-variable-rasterization-rate-content).

## See also

### Rasterization settings
- [Rendering at different rasterization rates](https://developer.apple.com/documentation/metal/rendering-at-different-rasterization-rates) — Configure a rasterization rate map to vary rasterization rates depending on the amount of detail needed.
- [Creating a rasterization rate map](https://developer.apple.com/documentation/metal/creating-a-rasterization-rate-map) — Define the rasterization rates for each part of your render target.
- [Scaling variable rasterization rate content](https://developer.apple.com/documentation/metal/scaling-variable-rasterization-rate-content) — Use the rate map data to scale the content to fill your destination texture.
- [MTLRasterizationRateMapDescriptor](https://developer.apple.com/documentation/metal/mtlrasterizationratemapdescriptor) — An object that you use to configure new rasterization rate maps.
- [MTLRasterizationRateMap](https://developer.apple.com/documentation/metal/mtlrasterizationratemap) — A compiled read-only instance that determines how to apply variable rasterization rates when rendering.
- [MTLCoordinate2D](https://developer.apple.com/documentation/metal/mtlcoordinate2d) — A coordinate in the viewport.
- [MTLCoordinate2DMake(_:_:)](https://developer.apple.com/documentation/metal/mtlcoordinate2dmake(_:_:)) — Returns a new 2D point with the specified coordinates.
