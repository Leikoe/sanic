# Optimizing texture data

*Article*

<https://developer.apple.com/documentation/metal/optimizing-texture-data>

Optimize a texture’s data to improve GPU or CPU access.

## Overview

By default, Metal attempts to optimize a texture’s data for both GPU and CPU memory operations based on the texture’s storage mode and usage options. You can improve a texture’s performance on the GPU or CPU by optimizing the texture’s data for either use case. You can also opt out of optimization altogether. Optimizing a texture’s performance for one use can decrease that texture’s performance for another.

Before optimizing texture data, carefully consider the storage modes and usage options for your textures. For guidance on resource storage modes, see [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes). For guidance on texture usage options, see [MTLTextureUsage](https://developer.apple.com/documentation/metal/mtltextureusage).

> **Note:**
>  Metal may not be able to optimize some textures for specific hardware and ignores optimization API calls for those textures.

### Optimize texture data for GPU access

By default, Metal attempts to optimize texture data for GPU access if it meets any of these conditions:

- You create the texture with an [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) mode.

- You create the texture with an [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) option.

If the texture doesn’t meet any of these conditions, you can optimize your texture data explicitly. After you create your texture and populate its contents, encode and commit an [optimizeContentsForGPUAccess(texture:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforgpuaccess(texture:)) or [optimizeContentsForGPUAccess(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforgpuaccess(texture:slice:level:)) command.

```swift
// Create the first texture.
let texture1GPUOptimized: MTLTexture! = nil
...

// Put content in the texture.
...

// Create a command buffer to submit work to the GPU.
let commandBuffer: MTLCommandBuffer! = commandQueue.makeCommandBuffer()

// Optimize the texture for GPU access by encoding a blit command.
let blitEncoder: MTLBlitCommandEncoder! = commandBuffer.makeBlitCommandEncoder()
blitEncoder.optimizeContentsForGPUAccess(texture: texture1GPUOptimized)

// End the encoding.
blitEncoder.endEncoding()

// Add a completion handler.
commandBuffer.addCompletedHandler {_ in
    // The GPU can now optimally access the contents of texture 1.
    ...
}

// Commit the command buffer to the command queue.
commandBuffer.commit()
```

```objective-c
// Create the first texture.
id <MTLTexture> texture1GPUOptimized;
...

// Put content in the texture.
...

// Create a command buffer to submit work to the GPU.
id <MTLCommandBuffer> commandBuffer = [commandQueue commandBuffer];

// Optimize the texture for GPU access by encoding a blit command.
id <MTLBlitCommandEncoder> blitEncoder = [commandBuffer blitCommandEncoder];
[blitEncoder optimizeContentsForGPUAccess:texture1GPUOptimized];

// End the encoding.
[blitEncoder endEncoding];

// Add a completion handler.
[commandBuffer addCompletedHandler:^(id<MTLCommandBuffer> commandBuffer) {
    // The GPU can now optimally access the contents of texture 1.
    ...
}];

// Commit the command buffer to the command queue.
[commandBuffer commit];
```

To optimize a drawable from an [MTKView](https://developer.apple.com/documentation/MetalKit/MTKView) for GPU access, set the view’s [framebufferOnly](https://developer.apple.com/documentation/MetalKit/MTKView/framebufferOnly) property to [true](https://developer.apple.com/documentation/Swift/true). This property configures the texture exclusively as a render target and displayable resource.

### Optimize texture data for CPU access

By default, Metal attempts to optimize texture data for CPU access if it meets both of these conditions:

- You create the texture with an [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) or [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed) mode.

- You write to the texture with the [replace(region:mipmapLevel:withBytes:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:withbytes:bytesperrow:)) or [replace(region:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:slice:withbytes:bytesperrow:bytesperimage:)) method.

If you don’t meet both of these conditions, you can optimize your texture data explicitly. After you create your texture and populate its contents, encode and commit an [optimizeContentsForCPUAccess(texture:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforcpuaccess(texture:)) or [optimizeContentsForCPUAccess(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforcpuaccess(texture:slice:level:)) command.

```swift
// Create a second texture.
let texture2CPUOptimized: MTLTexture! = nil
...

// Put content in the texture.
...

// Create a command buffer to submit work to the GPU.
let commandBuffer: MTLCommandBuffer! = commandQueue.makeCommandBuffer()

// Optimize the texture for CPU access by encoding a blit command.
let blitEncoder: MTLBlitCommandEncoder! = commandBuffer.makeBlitCommandEncoder()
blitEncoder.optimizeContentsForCPUAccess(texture: texture2CPUOptimized)

// End encoding and commit it to the command buffer with add a completion handler.
blitEncoder.endEncoding()
commandBuffer.addCompletedHandler {_ in
    // The CPU can now optimally access the contents of texture 2.
    ...
}

// Commit the command buffer to the command queue.
commandBuffer.commit()
```

```objective-c
// Create a second texture.
id <MTLTexture> texture2CPUOptimized;
...

// Put content in the texture.
...

// Create a command buffer to submit work to the GPU.
id <MTLCommandBuffer> commandBuffer = [commandQueue commandBuffer];

// Optimize the texture for CPU access by encoding a blit command.
id <MTLBlitCommandEncoder> blitEncoder = [commandBuffer blitCommandEncoder];
[blitEncoder optimizeContentsForCPUAccess:texture2CPUOptimized];

// End encoding and commit it to the command buffer with add a completion handler.
[blitEncoder endEncoding];
[commandBuffer addCompletedHandler:^(id<MTLCommandBuffer> commandBuffer) {
    // The CPU can now optimally access the contents of texture 2.
    ...
}];

// Commit the command buffer to the command queue.
[commandBuffer commit];
```

### Apply lossless compression to a texture on Apple GPUs

Lossless compression is a specific form of GPU optimization that Metal applies to a texture without discarding any of its data. Memory operations with textures that apply lossless compression typically need less memory bandwidth than equivalent memory operations with the same texture without compression. However, the overall memory footprint of a texture with lossless compression might increase slightly because it needs to store compression metadata. On devices that support [MTLGPUFamily.apple5](https://developer.apple.com/documentation/metal/mtlgpufamily/apple5), Metal attempts to apply lossless compression to a texture if it meets the following conditions:

- The texture’s pixel format doesn’t apply block-compression, such as PVRTC, ASTC, or BC.

- The texture’s usage options don’t include [unknown](https://developer.apple.com/documentation/metal/mtltextureusage/unknown), [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite), or [pixelFormatView](https://developer.apple.com/documentation/metal/mtltextureusage/pixelformatview).

- The texture doesn’t use any underlying [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance, such as a texture that comes from a buffer’s [makeTexture(descriptor:offset:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtlbuffer/maketexture(descriptor:offset:bytesperrow:)) method.

Additionally, if you meet both of the following conditions, you can optimize your texture data explicitly so Metal can apply lossless compression:

- You create the texture with an [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) mode.

- You write to the texture with the [replace(region:mipmapLevel:withBytes:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:withbytes:bytesperrow:)) or [replace(region:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)](https://developer.apple.com/documentation/metal/mtltexture/replace(region:mipmaplevel:slice:withbytes:bytesperrow:bytesperimage:)) method.

For guidance, see [Optimize texture data for GPU access](https://developer.apple.com/documentation/metal/optimizing-texture-data#Optimize-texture-data-for-GPU-access).

### Opt out of texture data optimization for GPU access

In some cases, your texture data may benefit from opting out of optimization for GPU access, for example, when optimization regresses your app’s performance (particularly for render target read-backs on the CPU).

First, create a texture descriptor and set its [allowGPUOptimizedContents](https://developer.apple.com/documentation/metal/mtltexturedescriptor/allowgpuoptimizedcontents) property to [false](https://developer.apple.com/documentation/Swift/false).

```swift
let textureDescriptor = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .rgba8Unorm,
                                                                 width: 512,
                                                                 height: 512,
                                                                 mipmapped: false)

// Don't allow the the GPU to optimize the texture.
textureDescriptor.allowGPUOptimizedContents = false
```

```objective-c
MTLTextureDescriptor *textureDescriptor =
 [MTLTextureDescriptor texture2DDescriptorWithPixelFormat:MTLPixelFormatRGBA8Unorm
                                                    width:512
                                                   height:512
                                                mipmapped:NO];

// Don't allow the the GPU to optimize the texture.
textureDescriptor.allowGPUOptimizedContents = NO;
```

Then, set the texture descriptor’s [storageMode](https://developer.apple.com/documentation/metal/mtltexturedescriptor/storagemode) property to [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) or [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed).

```swift
// Set the texture descriptor's storage mode to `shared` or `managed` based on the GPU family.
if device.supportsFamily(.apple1) {
    textureDescriptor.storageMode = .shared
} else {
    textureDescriptor.storageMode = .managed
}
```

```objective-c
// Set the texture descriptor's storage mode to `shared` or `managed` based on the GPU family.

if ([device supportsFamily:MTLGPUFamilyApple1]) {
    textureDescriptor.storageMode = MTLStorageModeShared;
} else {
    textureDescriptor.storageMode = MTLStorageModeManaged;
}
```

Finally, create a texture from the texture descriptor.

```swift
// Create a texture using the texture descriptor.
let texture = device.makeTexture(descriptor: textureDescriptor)
```

```objective-c
// Create a texture using the texture descriptor.
id <MTLTexture> texture = [device newTextureWithDescriptor:textureDescriptor];
```

## See also

### Texture basics
- [Understanding color-renderable pixel format sizes](https://developer.apple.com/documentation/metal/understanding-color-renderable-pixel-format-sizes) — Know the size limits of color render targets in Apple GPUs based on the target’s pixel format.
- [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) — A resource that holds formatted image data.
- [MTLTextureCompressionType](https://developer.apple.com/documentation/metal/mtltexturecompressiontype)
- [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) — An instance that you use to configure new Metal texture instances.
- [MTKTextureLoader](https://developer.apple.com/documentation/MetalKit/MTKTextureLoader) — An object that creates textures from existing data in common image formats.
- [MTLSharedTextureHandle](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle) — A texture handle that can be shared across process address space boundaries.
- [MTLPixelFormat](https://developer.apple.com/documentation/metal/mtlpixelformat) — The data formats that describe the organization and characteristics of individual pixels in a texture.
