# HDR content

<https://developer.apple.com/documentation/metal/hdr-content>

Take advantage of high dynamic range to present more vibrant colors in your apps and games.

## Overview

High dynamic range (HDR) content has a wider range of brightness levels than standard definition content. Certain displays, which macOS refers to as extended dynamic range (EDR) displays, can physically replicate those extra brightness values on a screen. You can use Metal to detect EDR displays and work with HDR content, such as from a video asset or directly from your app.

## Topics

### High dynamic range content
- [Processing HDR images with Metal](https://developer.apple.com/documentation/metal/processing-hdr-images-with-metal) — Implement a post-processing pipeline using the latest features on Apple GPUs.
- [Displaying HDR content in a Metal layer](https://developer.apple.com/documentation/metal/displaying-hdr-content-in-a-metal-layer) — Bring your high dynamic range (HDR) content to compatible Mac displays.
- [Determining support for EDR values](https://developer.apple.com/documentation/metal/determining-support-for-edr-values) — Check whether a display supports EDR.
- [Using color spaces to display HDR content](https://developer.apple.com/documentation/metal/using-color-spaces-to-display-hdr-content) — Use a color space when you don’t need to edit or process the pixel data.
- [Using system tone mapping on video content](https://developer.apple.com/documentation/metal/using-system-tone-mapping-on-video-content) — Use EDR metadata to apply the default system tone mapping to a layer.
- [Performing your own tone mapping](https://developer.apple.com/documentation/metal/performing-your-own-tone-mapping) — Apply your own tone mapping to get the exact behavior you want.
- [Implementing tone mapping on reference displays](https://developer.apple.com/documentation/metal/implementing-tone-mapping-on-reference-displays) — Detect reference displays and keep your content within the capabilities of the display hardware.

## See also

### Presentation
- [Managing your game window for Metal in macOS](https://developer.apple.com/documentation/metal/managing-your-game-window-for-metal-in-macos) — Set up a window and view for optimally displaying your Metal content.
- [Managing your Metal app window in iPadOS](https://developer.apple.com/documentation/metal/managing-your-metal-app-window-in-ipados) — Set up a window that handles dynamically resizing your Metal content.
- [Adapting your game interface for smaller screens](https://developer.apple.com/documentation/metal/adapting-your-game-interface-for-smaller-screens) — Make text legible on all devices the player chooses to run your game on.
- [Onscreen presentation](https://developer.apple.com/documentation/metal/onscreen-presentation) — Show the output from a GPU’s rendering pass to the user in your app.
