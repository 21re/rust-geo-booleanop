# Boolean operations on geo shapes

This is an implementation of the [Martinez-Rueda Polygon Clipping Algorithm](http://www.cs.ucr.edu/~vbz/cs230papers/martinez_boolean.pdf) in rust to integrate smoothly into the already exsting geospatial library  [georust/geo](https://github.com/georust/geo).

In fact the implementation closely follows the "reference" implementation in JavaScript: [https://github.com/w8r/martinez](https://github.com/w8r/martinez). Most of the concepts and fixtures have been taken from there.

At the moment the implementation contains is own splay tree implementation (adapted from [https://github.com/alexcrichton/splay-rs](https://github.com/alexcrichton/splay-rs)) as the JavaScript implementation also uses a splay-tree. This might be refactored out in the future in favor of the standard collection types (like BTreeSet).

