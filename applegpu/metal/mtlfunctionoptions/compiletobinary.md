# compileToBinary

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionoptions/compiletobinary>

An option that instructs the compiler to generate a binary format for dynamic linking.

## Declaration

```swift
static var compileToBinary: MTLFunctionOptions { get }
```

## See also

### Function compilation options
- [failOnBinaryArchiveMiss](https://developer.apple.com/documentation/metal/mtlfunctionoptions/failonbinaryarchivemiss) — An option that instructs the compiler to return an error when a GPU function isn’t in a binary archive.
- [pipelineIndependent](https://developer.apple.com/documentation/metal/mtlfunctionoptions/pipelineindependent) — An option that generates the same function handle across all pipeline states that link a function, which lets you share function tables across pipeline states.
- [storeFunctionInMetalPipelinesScript](https://developer.apple.com/documentation/metal/mtlfunctionoptions/storefunctioninmetalpipelinesscript) — An option that instructs the compiler to store function information for inspecting binary archives.
- [storeFunctionInMetalScript](https://developer.apple.com/documentation/metal/mtlfunctionoptions/storefunctioninmetalscript) — An option that instructs the compiler to store function information for inspecting binary archives.
