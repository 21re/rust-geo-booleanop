# Test framework usage notes

On top of unit tests, the algorithm comes with a large suite of integration tests.
Test cases are defined in `fixtures/generic_test_cases`. Each test case is a GeoJSON
file containing a `FeatureCollection` with the following semantics:

- The first two features are the subject and clipping polygon.
- The remaining features define expected test results. The `properties.operation` field is
  used to specify the operation (intersection, union, difference, xor) corresponding
  to the result. The `properties.comment` field may contain additional notes about a
  test case, e.g. if the test case currently still has issues.


## Visualizing test result contents

To inspect if the input and expected results of a test case make sense, there is a
small Python helper script in `scripts/plot_test_cases.py`. It allows both exporting
multiple test cases into a summary PDF, or showing interactive plots with zooming/panning
functionality.

```
usage: Tool to plot content test case files. [-h] [-i] [--pngs] [-o OUTPUT]
                                             <TEST-CASE-FILE>
                                             [<TEST-CASE-FILE> ...]

positional arguments:
  <TEST-CASE-FILE>      test case GeoJSON file(s) to plot

optional arguments:
  -h, --help            show this help message and exit
  -i, --interactive     whether to show interactive plot windows
  --pngs                whether to generate individual PNGs for each plot
  -o OUTPUT, --output OUTPUT
                        PDF output file name (default: test_cases.pdf)
```


## Running tests

- The standard `cargo test` runs all tests. The test suite uses the
  [`pretty-assertions`](https://github.com/colin-kiegel/rust-pretty-assertions) crate,
  to simplify finding problems in case of failing tests.

- For debugging a single test case file, there is also
  `cargo run <PATH_TO_TEST_CASE_GEOJSON`, which runs the test case and immediately
  shows an interactive visualization.


## Updating the test cases

After making changes to the algorithm, the expected results of the test cases may require
updating. This can be done by running `REGEN=true cargo test`. In this mode all existing
test cases are re-written with the current test output. Since this mode does not perform
output checks, test execution is marked as failed in the end to avoid accidentally passing
tests.
