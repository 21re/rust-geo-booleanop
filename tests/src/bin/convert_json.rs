extern crate geo_booleanop_tests;

use geojson::GeoJson;

use geo_booleanop_tests::helper::load_fixture_from_path;
use geo_booleanop_tests::compact_geojson::write_compact_geojson;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename_in = &args[1];
    let filename_out = &args[2];

    let geojson = load_fixture_from_path(&filename_in);

    let features = match geojson {
        GeoJson::FeatureCollection(collection) => collection.features,
        _ => panic!("Fixture is not a feature collection"),
    };

    write_compact_geojson(&features, &filename_out);
}