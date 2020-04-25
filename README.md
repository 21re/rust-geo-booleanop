[![Build Status](https://travis-ci.org/21re/rust-geo-booleanop.svg?branch=master)](https://travis-ci.org/21re/rust-geo-booleanop)
[![crates.io](https://img.shields.io/crates/v/geo-booleanop.svg)](https://crates.io/crates/geo-booleanop)


# Boolean operations on geo shapes

This is an implementation of the [Martinez-Rueda Polygon Clipping Algorithm](http://www.cs.ucr.edu/~vbz/cs230papers/martinez_boolean.pdf) in rust to integrate smoothly into the already exsting geospatial library  [georust/geo](https://github.com/georust/geo).

In fact the implementation closely follows the "reference" implementation in JavaScript: [https://github.com/w8r/martinez](https://github.com/w8r/martinez). Most of the concepts and fixtures have been taken from there.

At the moment the implementation contains its own splay tree implementation (adapted from [https://github.com/alexcrichton/splay-rs](https://github.com/alexcrichton/splay-rs)) as the JavaScript implementation also uses a splay-tree. This might be refactored out in the future in favor of the standard collection types (like BTreeSet).

# IMPORTANT: How to report bugs

Please be aware (so far) this implementation is based on the JavaScript version. If you find a bug (i.e. two polygons not producing the expected result), chances are that the original algorithm has the same problem. So please first check with [https://github.com/w8r/martinez](https://github.com/w8r/martinez) and file a report there. Once there is a fix I will happily backport it to the rust version.

If you do not know how to do that (You understand rust but not javascript? ... I mean ... seriously?), you may take a look at this example: https://gist.github.com/untoldwind/e95b7eff8ad61527a5dc4bdd889169b0

I.e. just create `package.json`, ìnsert your example coordinates in `main.js` and then do `npm install` followed by `node main.js`

# Usage

Pretty straightforward:

```
geo-booleanop = "0.2.1"
```

```rust
extern create geo;
extern crate geo_booleanop;

use geo_booleanop::boolean::BooleanOp;

fn main() {
    let poly1 : geo::Polygon<f64> = ...
    let poly2 : geo::Polygon<f64> = ...

    let intersect = poly1.intersection(&poly2);
    let union = poly1.union(&poly2);
    let diff = poly1.difference(&poly2);
    let xor = poly1.xor(&poly2);

    ...
}
```

MultiPolygon is supported as well.
