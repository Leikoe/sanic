# Setting load and store actions

*Article*

<https://developer.apple.com/documentation/metal/setting-load-and-store-actions>

Set actions that define how a render pass loads and stores a render target.

## Overview

[MTLLoadAction](https://developer.apple.com/documentation/metal/mtlloadaction) and [MTLStoreAction](https://developer.apple.com/documentation/metal/mtlstoreaction) values allow you to define how a render pass loads and stores your [MTLRenderPassAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor) objects. By choosing appropriate actions for your render targets, you can avoid costly and unnecessary work at the start (load) or end (store) of a render pass.

Set a render targetʼs texture on its [texture](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/texture) property. Then, set its actions on its [loadAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/loadaction) and [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) properties:

```swift
let renderPassDescriptor = MTLRenderPassDescriptor()

// Color render target
renderPassDescriptor.colorAttachments[0].texture = colorTexture
renderPassDescriptor.colorAttachments[0].loadAction = .clear
renderPassDescriptor.colorAttachments[0].storeAction = .store

// Depth render target
renderPassDescriptor.colorAttachments[0].texture = depthTexture
renderPassDescriptor.colorAttachments[0].loadAction = .dontCare
renderPassDescriptor.colorAttachments[0].storeAction = .dontCare

// Stencil render target
renderPassDescriptor.colorAttachments[0].texture = stencilTexture
renderPassDescriptor.colorAttachments[0].loadAction = .dontCare
renderPassDescriptor.colorAttachments[0].storeAction = .dontCare
```

```objective-c
MTLRenderPassDescriptor *renderPassDescriptor = [MTLRenderPassDescriptor renderPassDescriptor];

// Color render target
renderPassDescriptor.colorAttachments[0].texture = colorTexture;
renderPassDescriptor.colorAttachments[0].loadAction = MTLLoadActionClear;
renderPassDescriptor.colorAttachments[0].storeAction = MTLStoreActionStore;

// Depth render target
renderPassDescriptor.depthAttachment.texture = depthTexture;
renderPassDescriptor.depthAttachment.loadAction = MTLLoadActionDontCare;
renderPassDescriptor.depthAttachment.storeAction = MTLStoreActionDontCare;

// Stencil render target
renderPassDescriptor.stencilAttachment.texture = stencilTexture;
renderPassDescriptor.stencilAttachment.loadAction = MTLLoadActionDontCare;
renderPassDescriptor.stencilAttachment.storeAction = MTLStoreActionDontCare;
```

### Choose a load action

Several options are available, depending on which of the following scenarios describes your render targetʼs loading needs.

**You donʼt need the previous contents of the render target and you render to all of its pixels.** Choose [MTLLoadAction.dontCare](https://developer.apple.com/documentation/metal/mtlloadaction/dontcare). This action incurs no cost, and pixel values are always undefined at the start of the render pass.

![image](https://docs-assets.developer.apple.com/published/7cbe7d6210b0c950a2e64ee927f1cf3b/setting-load-and-store-actions-6%402x.png)

**You donʼt need the previous contents of the render target and you render to only some of its pixels.** Choose [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear). This action incurs the cost of writing the render targetʼs clear value to each pixel.

![image](https://docs-assets.developer.apple.com/published/cc14d2ba5f3ba032a79daa39c2131125/setting-load-and-store-actions-5%402x.png)

**You do need the previous contents of the render target and you render to only some of its pixels.** Choose [MTLLoadAction.load](https://developer.apple.com/documentation/metal/mtlloadaction/load). This action incurs the cost of loading the previous values of each pixel from memory. This action is significantly slower than [MTLLoadAction.dontCare](https://developer.apple.com/documentation/metal/mtlloadaction/dontcare) or [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear).

![image](https://docs-assets.developer.apple.com/published/d46146edb2f07bd96ccd8cb2a93b51d2/setting-load-and-store-actions-1%402x.png)

> **Note:**
>  You canʼt choose [MTLLoadAction.load](https://developer.apple.com/documentation/metal/mtlloadaction/load) for a memoryless render target because it isnʼt backed by system memory. For more information about memoryless render targets, see [Choosing a resource storage mode for Apple GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus).

### Choose a store action

Several options are available, depending on which of the following scenarios describes your render targetʼs storage needs.

**You donʼt need to preserve the contents of the render target.** Choose [MTLStoreAction.dontCare](https://developer.apple.com/documentation/metal/mtlstoreaction/dontcare). This action incurs no cost, and pixel values are always undefined at the end of the render pass. Choose this action for intermediary render targets that you use within the render pass, but you donʼt need afterward. This is typically the correct action for depth and stencil render targets.

![image](https://docs-assets.developer.apple.com/published/41f424b40ac240d00812b4718c0f1160/setting-load-and-store-actions-8%402x.png)

**You do need to preserve the contents of the render target.** Choose [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store). This action incurs the cost of storing the values of each pixel to memory. This is always the correct action for drawables.

![image](https://docs-assets.developer.apple.com/published/85088cb54304cfe0edef050dd5a1c742/setting-load-and-store-actions-7%402x.png)

**Your render target is a multisample texture.** When you perform multisampling, you decide whether to store the render targetʼs multisampled or resolved data. Multisampled data is stored in the render targetʼs [texture](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/texture) property. Resolved data is stored in the render targetʼs [resolveTexture](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvetexture) property. Refer to this table to choose a store action when multisampling:

| Multisampled data stored | Resolved data stored | Resolve texture required | Required store action |
|---|---|---|---|
| Yes | Yes | Yes | [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve) |
| Yes | No | No | [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store) |
| No | Yes | Yes | [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve) |
| No | No | No | [MTLStoreAction.dontCare](https://developer.apple.com/documentation/metal/mtlstoreaction/dontcare) |

To store and resolve a multisample texture in a single render pass, always choose the [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve) action and use a single render command encoder.

**You need to defer your storage choice.** In some cases, you might not know which store action to use for a particular render target until you gather more render pass information. To defer your store action choice, set the temporary [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value when you create your [MTLRenderPassAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor) object. Setting an unknown store action may avoid potential costs incurred by setting another store action prematurely. However, you need to specify a valid store action before you finish encoding your render pass; otherwise, an error occurs.

> **Note:**
>  You canʼt choose [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store) or [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve) for a memoryless render target because it isnʼt backed by system memory. For more information about memoryless render targets, see [Choosing a resource storage mode for Apple GPUs](https://developer.apple.com/documentation/metal/choosing-a-resource-storage-mode-for-apple-gpus).

### Evaluate actions between render passes

You can use the same render targets across multiple render passes. Several load and store combinations are possible for the same render target between any two render passes, depending on which of the following scenarios describes your render targetʼs needs from one render pass to another.

**You donʼt need the previous contents of a render target in the next render pass.** In the first render pass, choose [MTLStoreAction.dontCare](https://developer.apple.com/documentation/metal/mtlstoreaction/dontcare) to avoid storing the contents of the render target. In the second render pass, choose [MTLLoadAction.dontCare](https://developer.apple.com/documentation/metal/mtlloadaction/dontcare) or [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear) to avoid loading the contents of the render target.

![image](https://docs-assets.developer.apple.com/published/194bfae826cd6005bd66aa17d5d7a29a/setting-load-and-store-actions-2%402x.png)

![image](https://docs-assets.developer.apple.com/published/37cea4d54747a07c9abc09a62b25761c/setting-load-and-store-actions-3%402x.png)

**You do need the previous contents of a render target in the next render pass.** In the first render pass, choose [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store), [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve), or [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve) to store the contents of the render target. In the second render pass, choose [MTLLoadAction.load](https://developer.apple.com/documentation/metal/mtlloadaction/load) to load the contents of the render target.

![image](https://docs-assets.developer.apple.com/published/0909b9c8f011dff122de8ecbb7208c80/setting-load-and-store-actions-4%402x.png)

## See also

### Applying rendering techniques
- [Drawing a triangle with Metal 4](https://developer.apple.com/documentation/metal/drawing-a-triangle-with-metal-4) — Render a colorful, rotating 2D triangle by running draw commands with a render pipeline on a GPU.
- [Customizing render pass setup](https://developer.apple.com/documentation/metal/customizing-render-pass-setup) — Render into an offscreen texture by creating a custom render pass.
- [Improving rendering performance with vertex amplification](https://developer.apple.com/documentation/metal/improving-rendering-performance-with-vertex-amplification) — Run draw commands that render to different outputs using the same vertex data multiple times.
