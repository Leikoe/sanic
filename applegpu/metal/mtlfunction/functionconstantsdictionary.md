# functionConstantsDictionary

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunction/functionconstantsdictionary>

A dictionary of function constants for a specialized function.

## Declaration

```swift
var functionConstantsDictionary: [String : MTLFunctionConstant] { get }
```

## Discussion

This property returns a dictionary of the function constants that you need to provide to specialize this function. This property returns an empty dictionary if this function is already specialized or doesn’t declare any function constants.

To create the specialized function, set these constant values in a new [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues) object and call the [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) method.
