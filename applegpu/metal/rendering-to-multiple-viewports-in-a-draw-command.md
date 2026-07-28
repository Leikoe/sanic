# Rendering to multiple viewports in a draw command

*Article*

<https://developer.apple.com/documentation/metal/rendering-to-multiple-viewports-in-a-draw-command>

Select viewports and their corresponding scissor rectangles in your vertex shader.

## Overview

A viewport defines a subsection of the render targets that you want a drawing command to render into. Using viewport selection, you provide multiple viewports for a drawing command, and dynamically choose one of these viewports for each primitive rendered by the drawing command. Viewport selection makes it easier to consolidate rendering to multiple viewports into fewer drawing commands. For example, you might use viewport selection when rendering stereo imagery or other images whose content is rendered to multiple parts of the render target.

![image](https://docs-assets.developer.apple.com/published/c028ba3b6b343c64f8f0ff8c12debc4c/rendering-to-multiple-viewports-in-a-draw-command-1%402x.png)

### Check the device object for support for multiple viewports

All GPUs in the macOS family support multiple viewports. Multiple viewports are available in the Apple GPU family starting with family 5. Test for support using the code below:

```swift
func supportsMultipleViewports() -> Bool {
    return device.supportsFamily(MTLGPUFamily.mac2) || device.supportsFamily(MTLGPUFamily.apple5)
}
```

```objective-c
- (Boolean) supportsMultipleViewports
{
    return [_device supportsFamily: MTLGPUFamilyMac1 ] ||
           [_device supportsFamily: MTLGPUFamilyApple5 ];
}
```

For the maximum number of viewports you can use with each GPU family, see:

- [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf)

- [Metal feature set tables (Numbers)](https://developer.apple.com/metal/metal-feature-set-tables.zip)

### Add viewport selection to your vertex shader

To specify which viewport a primitive should be rendered into, add a vertex output with the `[[viewport_array_index]]` attribute. Your vertex shader needs to set this value so that Metal knows which viewport to render into.

The example below uses instanced rendering to primitives to multiple viewports. It adds a `viewPort` field to the vertex output to specify the target slice. The target viewport is passed in as part of the per-instance properties, and copied to the vertex output.

```metal
typedef struct
{
    ...
    uint   viewport [[viewport_array_index]];
} ColorInOut;

vertex ColorInOut vertexTransform (
    const Vertex in [[ stage_in ]],
    const uint   instanceId                       [[ instance_id ]],
    const device InstanceParams* instanceParams   [[ buffer ]],
{
    ColorInOut out;
    out.viewport = instanceParams[instanceId].viewport;
    ...
}
```

Your vertex function needs to return the same index for all vertices that make up any given primitive.

The rasterization stage uses the selected viewport and associated scissor rectangle to transform the vertex outputs and then passes the data over to the fragment stage. If you need to know which viewport is being rendered to inside your fragment shader, you can reference the same field that you set in the vertex output.

### Specify viewports and scissor rectangles in your draw command

Call [setViewports(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewports(_:)) to specify multiple viewports and [setScissorRects(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrects(_:)) to specify scissor rectangles:

```swift
renderEncoder.setViewports(viewPortsArray)
renderEncoder.setScissorRects(scissorRectsArray)
```

```objective-c
[renderEncoder setViewports:viewPortsArray count:4];
[renderEncoder setScissorRects:scissorRectsArray count:4];
```

Specify the same number of scissor rectangles and viewports. Coordinate your code that encodes render commands with the code in your shaders such that the indices that your shader generates are within the range of provided values.

## See also

### Optimizing techniques
- [Specifying drawing and dispatch arguments indirectly](https://developer.apple.com/documentation/metal/specifying-drawing-and-dispatch-arguments-indirectly) — Use indirect commands if you don’t know your draw or dispatch call arguments when you encode the command.
- [Rendering to multiple texture slices in a draw command](https://developer.apple.com/documentation/metal/rendering-to-multiple-texture-slices-in-a-draw-command) — Select a destination texture slice in your vertex shader.
