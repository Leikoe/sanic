# init(width:height:depth:)

*Initializer · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlsize/init(width:height:depth:)>

Creates a size instance with values for its width, height, and depth properties.

## Declaration

```swift
init(width: Int, height: Int, depth: Int)
```

## Parameters

- **width** — A value for the x-axis dimension.
- **height** — A value for the y-axis dimension. Pass `1` for sizes with one dimension.
- **depth** — A value for the z-axis dimension. Pass `1` for sizes with one or two dimensions.

## See also

### Creating a size instance
- [init()](https://developer.apple.com/documentation/metal/mtlsize/init()) — Creates a default size instance by setting the initial values for its width, height, and depth properties to zero.
- [MTLSizeMake(_:_:_:)](https://developer.apple.com/documentation/metal/mtlsizemake(_:_:_:)) — Creates a size instance with values for its width, height, and depth properties.
