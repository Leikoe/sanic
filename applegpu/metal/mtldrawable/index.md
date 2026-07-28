# MTLDrawable

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldrawable>

A displayable resource that can be rendered or written to.

## Declaration

```swift
protocol MTLDrawable : NSObjectProtocol
```

## Overview

Objects that implement this protocol are connected both to the Metal framework and an underlying display system (such as Core Animation) that’s capable of showing content onscreen. You use drawable objects when you want to render images using Metal and present them onscreen.

Don’t implement this protocol yourself; instead, see [CAMetalLayer](https://developer.apple.com/documentation/QuartzCore/CAMetalLayer), for a class that can create and manage drawable objects for you.

## Topics

### Identifying the drawable
- [drawableID](https://developer.apple.com/documentation/metal/mtldrawable/drawableid) — A positive integer that identifies the drawable.

### Presenting the drawable
- [present()](https://developer.apple.com/documentation/metal/mtldrawable/present()) — Presents the drawable onscreen as soon as possible.
- [present(afterMinimumDuration:)](https://developer.apple.com/documentation/metal/mtldrawable/present(afterminimumduration:)) — Presents the drawable onscreen as soon as possible after a previous drawable is visible for the specified duration.
- [present(at:)](https://developer.apple.com/documentation/metal/mtldrawable/present(at:)) — Presents the drawable onscreen at a specific host time.

### Getting presentation information
- [addPresentedHandler(_:)](https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler(_:)) — Registers a block of code to be called immediately after the drawable is presented.
- [presentedTime](https://developer.apple.com/documentation/metal/mtldrawable/presentedtime) — The host time, in seconds, when the drawable was displayed onscreen.

## See also

### Render pass outputs
- [MTLDrawablePresentedHandler](https://developer.apple.com/documentation/metal/mtldrawablepresentedhandler) — A block of code invoked after a drawable is presented.
