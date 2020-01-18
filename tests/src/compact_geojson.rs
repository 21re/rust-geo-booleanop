use geo_booleanop::boolean::BooleanOp;

use geo::{Coordinate, MultiPolygon, Polygon};
use geojson::{GeoJson, Feature, FeatureCollection, Value, Geometry};
use pretty_assertions::assert_eq;

use serde_json::json;

use std::fs::File;
use std::convert::TryInto;
use std::io::prelude::*;


fn indent_block(indent: i32, s: &str) -> String {
    let indent = " ".repeat(indent as usize);
    s.split("\n").enumerate().map(|(i, line)| {
        if i != 0 {
            indent.clone() + line //+ "\n"
        } else {
            line.to_string() //+ "\n"
        }
    }).collect::<Vec<_>>().join("\n")
}


trait WriteIndented {
    fn write_indented<S: AsRef<str>>(self, indent: i32, s: S);
    fn write_indented_with_substring(self, indent: i32, s: &str);
}

impl WriteIndented for &mut File {
    fn write_indented<S: AsRef<str>>(self, indent: i32, s: S) {
        let indent = " ".repeat(indent as usize);
        self.write(indent.as_bytes()).expect("Failed to write to file.");
        self.write(s.as_ref().as_bytes()).expect("Failed to write to file.");
    }

    fn write_indented_with_substring(self, indent: i32, s: &str) {
        let indent = " ".repeat(indent as usize);
        //let s = s.replace("\n", &("\n".to_string() + &indent));
        for line in s.split("\n") {
            if line.len() > 0 {
                self.write(indent.as_bytes()).expect("Failed to write to file.");
            }
            self.write(line.as_bytes()).expect("Failed to write to file.");
            self.write(b"\n").expect("Failed to write to file.");
        }
    }

}


fn write_polygon(polygon: &Vec<Vec<Vec<f64>>>, f: &mut File, indent: i32) {

    let mut write = |s: &str| {
        f.write_indented(indent, s);
    };

    let float_to_string = |x: f64| {
        let s = json!(x).to_string();
        if s.ends_with(".0") { s[..s.len()-2].to_string() } else { s }
    };

    //write("[");
    for (i, ring) in polygon.iter().enumerate() {
        write("[\n");
        for (j, point) in ring.iter().enumerate() {
            write(&format!(
                "  [{}, {}]{}\n",
                float_to_string(point[0]),
                float_to_string(point[1]),
                if j < ring.len() - 1 { "," } else {""},
            ));
        }
        if i < polygon.len() - 1 {
            write("],\n");
        } else {
            write("]\n");
        }

    }
    //write("]");
}

fn write_multi_polygon(polygons: &Vec<Vec<Vec<Vec<f64>>>>, f: &mut File, indent: i32) {

    //write("[");
    for (i, polygon) in polygons.iter().enumerate() {
        f.write_indented(indent, "[\n");
        write_polygon(polygon, f, indent + 2);
        if i < polygon.len() - 1 {
            f.write_indented(indent, "],\n");
        } else {
            f.write_indented(indent, "]\n");
        }

    }
    //write("]");
}

fn write_feature(feature: &Feature, f: &mut File, is_last: bool) {
    /*
    let header = indoc!(r#"
        {
          "geometry": {
            "coordinates": ["#);
    let footer = indoc!(format!(r#"
            ],
            "type": "Polygon"
          },
          "properties": {},
          "type": "Feature"
        }{}"#, 1234, if is_last { "" } else {","}));
    */

    //f.write_indented(4, header);
    f.write_indented(4, "{\n");
    f.write_indented(4, "  \"geometry\": {\n");
    f.write_indented(4, "    \"coordinates\": [\n");

    let geometry_value = feature.geometry.as_ref().expect("Feature must have 'geometry' property").value.clone();
    let geometry_type_name = match geometry_value {
        Value::Polygon(data) => {
            //let data: Polygon<f64> = geometry_value.try_into().unwrap();
            write_polygon(&data, f, 10);
            "Polygon"
        },
        Value::MultiPolygon(data) => {
            write_multi_polygon(&data, f, 10);
            "MultiPolygon"
        },
        _ => panic!("Feature must either be MultiPolygon or Polygon"),
    };

    //f.write_indented(4, &(footer.to_string() + if is_last { "" } else {","}));
    //f.write_indented(4, footer);

    let properties = feature.properties.as_ref().map_or(
        "{}\n".to_string(),
        |p| indent_block(6, &serde_json::to_string_pretty(&p).expect("Failed to convert properties to string.")),
    );

    f.write_indented(4, "    ],\n");
    f.write_indented(4, "    \"type\": \"".to_string() + geometry_type_name + "\"\n");
    f.write_indented(4, "  },\n");
    f.write_indented(4, "  \"properties\": ".to_string() + &properties + ",\n");
    f.write_indented(4, "  \"type\": \"Feature\"\n");
    if !is_last {
        f.write_indented(4, "},\n");
    } else {
        f.write_indented(4, "}\n");
    }
}


pub fn write_compact_geojson(features: &[Feature], filename: &str) {
    let output_geojson = GeoJson::FeatureCollection(FeatureCollection {
        bbox: None,
        features: features.to_vec(),
        foreign_members: None,
    });

    let mut f = File::create(filename).expect("Unable to create json file.");
    //serde_json::to_writer_pretty(f, &output_geojson).expect("Unable to write json file.");

    f.write_indented(0, "{\n");
    f.write_indented(0, "  \"features\": [\n");
    for (i, feature) in features.iter().enumerate() {
        write_feature(&feature, &mut f, i == features.len() - 1);
    }
    //f.write_indented(0, &footer);
    f.write_indented(0, "  ],\n");
    f.write_indented(0, "  \"type\": \"FeatureCollection\"\n");
    f.write_indented(0, "}\n");

}