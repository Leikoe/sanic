# GPU counters and counter sample buffers

*API Collection*

<https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers>

Retrieve runtime data from a GPU device by sampling one or more of its counters.

## Overview

A GPU *counter* ([MTLCounter](https://developer.apple.com/documentation/metal/mtlcounter)) is typically a hardware feature that tracks a specific performance metric, such as timestamps before and after an important rendering stage. A *counter set* ([MTLCounterSet](https://developer.apple.com/documentation/metal/mtlcounterset)) is a collection of related counters. A *counter sample buffer* ([MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer)) represents the memory where a GPU device stores the data for a specific counter set.

You can retrieve and inspect data from a GPU’s counter set with the following steps:

1. Inspect which GPU counter sets a GPU device supports (see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports)).

2. Make a counter sample buffer to store the data (see [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass)).

3. Instruct the GPU to save the counter set data to the buffer during a pass or an immediate mode command (see [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers)).

4. Transform the counter set data into a standard type (see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format)).

If you’re sampling data from a timestamp counter set ([timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp)), you may need to convert the timestamps from the GPU’s clock to the CPU’s clock. See [Converting GPU timestamps into CPU time](https://developer.apple.com/documentation/metal/converting-gpu-timestamps-into-cpu-time) for more information.

## Topics

### Counters and counter sets
- [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports) — Check whether a GPU produces the runtime performance data you want to sample.
- [MTLCounterSet](https://developer.apple.com/documentation/metal/mtlcounterset) — A collection of individual counters a GPU device supports for a counter set.
- [MTLCommonCounterSet](https://developer.apple.com/documentation/metal/mtlcommoncounterset) — The name of a specific counter set that a GPU device can support.
- [MTLCounter](https://developer.apple.com/documentation/metal/mtlcounter) — An individual counter a GPU device lists within one of its counter sets.
- [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) — The name of a specific counter that can appear in a GPU device’s counter sets.

### Counter sample buffers
- [Creating a counter sample buffer to store a GPU’s counter data during a pass](https://developer.apple.com/documentation/metal/creating-a-counter-sample-buffer-to-store-a-gpus-counter-data-during-a-pass) — Make a buffer that provides a place for a GPU to save its runtime performance metrics as it runs a pass.
- [MTLCounterSampleBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor) — A group of properties that configures the counter sample buffers you create with it.
- [MTLCounterSampleBuffer](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer) — A specialized memory buffer that stores a GPU’s counter set data.
- [Sampling GPU data into counter sample buffers](https://developer.apple.com/documentation/metal/sampling-gpu-data-into-counter-sample-buffers) — Retrieve a GPU’s counter data at a time the GPU supports.
- [MTLCounterDontSample](https://developer.apple.com/documentation/metal/mtlcounterdontsample) — A sentinel value that instructs an encoder to skip sampling a counter as the GPU runs the encoder’s pass.

### Counter sample data output
- [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format) — Inspect and use the data within a GPU’s counter sample buffer by resolving it into a standard format.
- [MTLCounterResultTimestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp) — The data structure for storing the data you resolve from a timestamp counter set.
- [MTLCounterResultStatistic](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic) — The data structure for storing the data you resolve from a statistic counter set.
- [MTLCounterResultStageUtilization](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization) — The data structure for storing the data you resolve from a stage-utilization counter set.
- [MTLCounterErrorValue](https://developer.apple.com/documentation/metal/mtlcountererrorvalue) — A sentinel value for an entry in a counter sample buffer that indicates the entry’s data is invalid.

### Timestamp data
- [Converting GPU timestamps into CPU time](https://developer.apple.com/documentation/metal/converting-gpu-timestamps-into-cpu-time) — Correlate GPU events with CPU timelines by calculating the CPU time equivalents for GPU timestamps.
- [MTLTimestamp](https://developer.apple.com/documentation/metal/mtltimestamp) — The number of nanoseconds for a point in absolute time or Mach absolute time.

### Counter sample buffer errors
- [MTLCounterSampleBufferError](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct) — The error codes that indicate why a GPU driver can’t create a counter sample buffer.

## See also

### Developer tools
- [Supporting Simulator in a Metal app](https://developer.apple.com/documentation/metal/supporting-simulator-in-a-metal-app) — Configure alternative render paths in your Metal app to enable running your app in Simulator.
- [Capturing Metal commands programmatically](https://developer.apple.com/documentation/metal/capturing-metal-commands-programmatically) — Invoke a Metal frame capture from your app, then save the resulting GPU trace to a file or view it in Xcode.
- [Logging shader debug messages](https://developer.apple.com/documentation/metal/logging-shader-debug-messages) — Print debugging messages that a shader generates using shader logging.
- [Developing Metal apps that run in Simulator](https://developer.apple.com/documentation/metal/developing-metal-apps-that-run-in-simulator) — Prototype and test your Metal apps in Simulator.
- [Improving your game’s graphics performance and settings](https://developer.apple.com/documentation/metal/improving-your-games-graphics-performance-and-settings) — Fix performance glitches and develop default settings for smooth experiences on Apple platforms using the powerful suite of Metal development tools.
- [Metal debugger](https://developer.apple.com/documentation/Xcode/Metal-debugger) — Debug and profile your Metal workload with a GPU trace.
- [Metal developer workflows](https://developer.apple.com/documentation/Xcode/Metal-developer-workflows) — Locate and fix issues related to your app’s use of the Metal API and GPU functions.
- [Metal debugging types](https://developer.apple.com/documentation/metal/metal-debugging-types) — Create capture managers and capture scopes, and review a GPU device’s log after it runs a command buffer.
