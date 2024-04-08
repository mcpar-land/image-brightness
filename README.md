A simple utility that takes an image and gives its average brightness across all pixels. optimized for speed.

Takes a single optional argument, the path to the image you want the brightness of. If no argument is provided, it uses stdin.

Outputs a float from `0.0` to `1.0`, where 0 is all black pixels and 1 is all white pixels.

This can also be used as a rust library.
