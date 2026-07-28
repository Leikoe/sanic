# addPresentedHandler(_:)

*Instance Method · iOS 10.3, iPadOS 10.3, Mac Catalyst 13.4, macOS 10.15.4, tvOS 10.2, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler(_:)>

Registers a block of code to be called immediately after the drawable is presented.

## Declaration

```swift
func addPresentedHandler(_ block: @escaping MTLDrawablePresentedHandler)
```

## Parameters

- **block** — A block of code to be invoked.

## Discussion

You can register multiple handlers for a single drawable object.

The following example code schedules a presentation handler that reads the [presentedTime](https://developer.apple.com/documentation/metal/mtldrawable/presentedtime) property and uses it to derive the interval between the last and current presentation times. From that information, it determines the app’s frame rate.

```swift
// Property declarations
var previousPresentedTime: CFTimeInterval = 0.0
/* ... */
// Render loop
currentDrawable.addPresentedHandler({ [weak self] drawable in
    guard let strongSelf = self else {
        return
    }
    let presentationDuration = drawable.presentedTime - strongSelf.previousPresentedTime
    let frameRate = 1.0/presentationDuration
    /* ... */
    strongSelf.previousPresentedTime = drawable.presentedTime
})
```

```objective-c
// Property declarations
@property (nonatomic) CFTimeInterval previousPresentedTime;
/* ... */
// Render loop
__block Renderer *strongSelf = self;
[view.currentDrawable addPresentedHandler:^(id<MTLDrawable> drawable) {
    CFTimeInterval presentationDuration = drawable.presentedTime - strongSelf.previousPresentedTime;
    CFTimeInterval frameRate = 1.0/presentationDuration;
    /* ... */
    strongSelf.previousPresentedTime = drawable.presentedTime;
}];
```

## See also

### Getting presentation information
- [presentedTime](https://developer.apple.com/documentation/metal/mtldrawable/presentedtime) — The host time, in seconds, when the drawable was displayed onscreen.
