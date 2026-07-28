# Metal debugging types

*API Collection*

<https://developer.apple.com/documentation/metal/metal-debugging-types>

Create capture managers and capture scopes, and review a GPU device’s log after it runs a command buffer.

## Topics

### Frame capture
- [MTLCaptureDescriptor](https://developer.apple.com/documentation/metal/mtlcapturedescriptor) — A configuration for a Metal capture session.
- [MTLCaptureManager](https://developer.apple.com/documentation/metal/mtlcapturemanager) — An instance you use to capture Metal command data in your app.
- [MTLCaptureDestination](https://developer.apple.com/documentation/metal/mtlcapturedestination) — The kinds of destinations for captured command data.
- [MTLCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturescope) — A type that can programmatically customize a GPU frame capture.

### Capture errors
- [MTLCaptureError](https://developer.apple.com/documentation/metal/mtlcaptureerror) — Errors returned by capture sessions.
- [MTLCaptureErrorDomain](https://developer.apple.com/documentation/metal/mtlcaptureerrordomain) — The error domain for capture errors.

### Shader logs
- [MTLFunctionLog](https://developer.apple.com/documentation/metal/mtlfunctionlog) — A log entry a Metal device generates when the it runs a command buffer.
- [MTLLogContainer](https://developer.apple.com/documentation/metal/mtllogcontainer-swift.struct) — A collection of logged messages, created when a Metal device runs a command buffer.

## See also

### Developer tools
- [Supporting Simulator in a Metal app](https://developer.apple.com/documentation/metal/supporting-simulator-in-a-metal-app) — Configure alternative render paths in your Metal app to enable running your app in Simulator.
- [Capturing Metal commands programmatically](https://developer.apple.com/documentation/metal/capturing-metal-commands-programmatically) — Invoke a Metal frame capture from your app, then save the resulting GPU trace to a file or view it in Xcode.
- [Logging shader debug messages](https://developer.apple.com/documentation/metal/logging-shader-debug-messages) — Print debugging messages that a shader generates using shader logging.
- [Developing Metal apps that run in Simulator](https://developer.apple.com/documentation/metal/developing-metal-apps-that-run-in-simulator) — Prototype and test your Metal apps in Simulator.
- [Improving your game’s graphics performance and settings](https://developer.apple.com/documentation/metal/improving-your-games-graphics-performance-and-settings) — Fix performance glitches and develop default settings for smooth experiences on Apple platforms using the powerful suite of Metal development tools.
- [Metal debugger](https://developer.apple.com/documentation/Xcode/Metal-debugger) — Debug and profile your Metal workload with a GPU trace.
- [Metal developer workflows](https://developer.apple.com/documentation/Xcode/Metal-developer-workflows) — Locate and fix issues related to your app’s use of the Metal API and GPU functions.
- [GPU counters and counter sample buffers](https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers) — Retrieve runtime data from a GPU device by sampling one or more of its counters.
