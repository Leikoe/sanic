# Storing data a pass makes with custom sample positions for a subsequent pass

*Article*

<https://developer.apple.com/documentation/metal/storing-data-a-pass-makes-with-custom-sample-positions-for-a-subsequent-pass>

Inform Metal when your app uses programmable sample positions for its depth render targets or copies MSAA depth data.

## Overview

A render or compute pass usually stores its target’s depth data in a compressed format. Any subsequent pass has to use the correct sample positions to decompress the data before reading it. You can store depth data in a representation that uses arbitrary sample positions (see [Positioning samples programmatically](https://developer.apple.com/documentation/metal/positioning-samples-programmatically)).

> **Important:**
>  You can sample depth positions programmatically only on devices that support programmable sample positions (see [areProgrammableSamplePositionsSupported](https://developer.apple.com/documentation/metal/mtldevice/areprogrammablesamplepositionssupported)).

When your app uses custom sampling positions, inform Metal by setting the [MTLRenderPassColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor) or [MTLRenderPassDepthAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdepthattachmentdescriptor) instance’s [storeActionOptions](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeactionoptions) property to [customSamplePositions](https://developer.apple.com/documentation/metal/mtlstoreactionoptions/customsamplepositions). This setting tells Metal that any subsequent pass that reads the attachment may not know the sample positions the current pass uses to generate the data. Examples of a pass that can use custom sample positions include the following:

- A fragment shader that uses unique, programmable sample positions

- A blit pass that copies MSAA depth data to another resource

In this scenario, Metal may decompress the depth render target and store the uncompressed data.

> **Tip:**
>  Improve the performance of a pass if its programmable sample positions are the same for the next pass by setting the descriptor’s [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) property to [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store) and clearing the [customSamplePositions](https://developer.apple.com/documentation/metal/mtlstoreactionoptions/customsamplepositions) option from the [storeActionOptions](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeactionoptions) property.

## See also

### Advanced multisampling
- [Positioning samples programmatically](https://developer.apple.com/documentation/metal/positioning-samples-programmatically) — Configure the position of samples when rendering to a multisampled render target.
