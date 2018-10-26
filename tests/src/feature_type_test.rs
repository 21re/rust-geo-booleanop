use super::helper::{fixture_multi_polygon, fixture_polygon};
use geo::Polygon;
use geo_booleanop::boolean::BooleanOp;

fn assert_clipping<F>(subject: &F, clipping: &Polygon<f64>, expected_name: &str)
where
    F: BooleanOp<f64, Polygon<f64>>,
{
    {
        let result = subject.intersection(clipping);
        let expected = fixture_multi_polygon(format!("feature_types/out/intersection/{}", expected_name).as_str());

        assert_eq!(result, expected);
    }
    {
        let result = subject.difference(clipping);
        let expected = fixture_multi_polygon(format!("feature_types/out/difference/{}", expected_name).as_str());

        assert_eq!(result, expected);
    }
    {
        let result = subject.union(clipping);
        let expected = fixture_multi_polygon(format!("feature_types/out/union/{}", expected_name).as_str());

        assert_eq!(result, expected);
    }
    {
        let result = subject.xor(clipping);
        let expected = fixture_multi_polygon(format!("feature_types/out/xor/{}", expected_name).as_str());

        assert_eq!(result, expected);
    }
}

#[test]
fn test_poly() {
    let clipping = fixture_polygon("feature_types/clipping_poly.geojson");
    let poly = fixture_polygon("feature_types/poly.geojson");

    assert_clipping(&poly, &clipping, "poly_to_clipping.geojson")
}

#[test]
fn test_poly_with_hole() {
    let clipping = fixture_polygon("feature_types/clipping_poly.geojson");
    let poly = fixture_polygon("feature_types/poly_with_hole.geojson");

    assert_clipping(&poly, &clipping, "poly_with_hole_to_clipping.geojson")
}

#[test]
fn test_multi_poly() {
    let clipping = fixture_polygon("feature_types/clipping_poly.geojson");
    let poly = fixture_multi_polygon("feature_types/multi_poly.geojson");

    assert_clipping(&poly, &clipping, "multi_poly_to_clipping.geojson")
}

#[test]
fn test_multi_poly_with_hole() {
    let clipping = fixture_polygon("feature_types/clipping_poly.geojson");
    let poly = fixture_multi_polygon("feature_types/multi_poly_with_hole.geojson");

    assert_clipping(&poly, &clipping, "multi_poly_with_hole_to_clipping.geojson")
}
