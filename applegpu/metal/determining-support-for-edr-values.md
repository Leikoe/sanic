# Determining support for EDR values

*Article*

<https://developer.apple.com/documentation/metal/determining-support-for-edr-values>

Check whether a display supports EDR.

## Overview

Discover whether a display supports EDR values by reading the [maximumPotentialExtendedDynamicRangeColorComponentValue](https://developer.apple.com/documentation/AppKit/NSScreen/maximumPotentialExtendedDynamicRangeColorComponentValue) property on an [NSScreen](https://developer.apple.com/documentation/AppKit/NSScreen) instance for that display. A value greater than `1.0` indicates that the display supports EDR values; otherwise, the display supports only SDR values.

This property’s value is independent of the current state of the display. It’s possible for a display to support EDR but to be unable to present those values right now. For information about the current state of the display, check the [maximumExtendedDynamicRangeColorComponentValue](https://developer.apple.com/documentation/AppKit/NSScreen/maximumExtendedDynamicRangeColorComponentValue) property.

## See also

### High dynamic range content
- [Processing HDR images with Metal](https://developer.apple.com/documentation/metal/processing-hdr-images-with-metal) — Implement a post-processing pipeline using the latest features on Apple GPUs.
- [Displaying HDR content in a Metal layer](https://developer.apple.com/documentation/metal/displaying-hdr-content-in-a-metal-layer) — Bring your high dynamic range (HDR) content to compatible Mac displays.
- [Using color spaces to display HDR content](https://developer.apple.com/documentation/metal/using-color-spaces-to-display-hdr-content) — Use a color space when you don’t need to edit or process the pixel data.
- [Using system tone mapping on video content](https://developer.apple.com/documentation/metal/using-system-tone-mapping-on-video-content) — Use EDR metadata to apply the default system tone mapping to a layer.
- [Performing your own tone mapping](https://developer.apple.com/documentation/metal/performing-your-own-tone-mapping) — Apply your own tone mapping to get the exact behavior you want.
- [Implementing tone mapping on reference displays](https://developer.apple.com/documentation/metal/implementing-tone-mapping-on-reference-displays) — Detect reference displays and keep your content within the capabilities of the display hardware.
