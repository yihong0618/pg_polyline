use pgrx::prelude::*;
use pgrx::pg_sys::Point;
use polyline;
use geo_types::{LineString, coord};

::pgrx::pg_module_magic!();

#[pg_extern]
fn polyline_encode(coords: pgrx::Array<Point<>>, precision: i32) -> String {
    // coord must be a two-dimensional array like [[x1, y1], [x2, y2], ...]
    // coord to vec![[x1, y1], [x2, y2], ...]
    // let res: LineString<f64> = vec![[9.9131118, 54.0702648], [9.9126013, 54.0702578]].into();
    let coords: Vec<_> = coords.iter_deny_null().map(|arr| {
        coord! {
            x: arr.x,
            y: arr.y,
        }
    }).collect();
    let res: LineString<f64> = LineString(coords.into());
    polyline::encode_coordinates(res, precision as u32).unwrap()
}


#[pg_extern]
fn polyline_decode(polyline_str: String,  precision: i32) -> Vec<Point<>> {
    polyline::decode_polyline(&polyline_str, precision as u32).unwrap().into_iter().map(|coord| {
        Point {
            x: coord.x,
            y: coord.y,
        }
    }).collect()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use pgrx::pg_sys::Point;

    #[pg_test]
    fn test_polyline_encode() {
        let s = Spi::get_one("select polyline_encode(ARRAY[point(-120.2, 38.5), point(-120.95, 40.7), point(-126.453, 43.252)], 6)");
        assert_eq!(s.unwrap(), "_izlhA~rlgdF_{geC~ywl@_kwzCn`{nI".to_string().into());
    }
    #[pg_test]
    fn test_polyline_decode() {
        let points = vec![
            Point { x: 0.2, y: 0.1 },
            Point { x: 0.4, y: 0.3 },
        ];
        assert_eq!(points, crate::polyline_decode("_ibE_seK_seK_seK".to_string(), 6));
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
