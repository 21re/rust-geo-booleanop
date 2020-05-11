use geo_types::{Coordinate, CoordinateType, LineString, MultiPolygon, Polygon};

pub mod compare_segments;
pub mod compute_fields;
mod connect_edges;
mod divide_segment;
pub mod fill_queue;
mod helper;
pub mod possible_intersection;
mod segment_intersection;
mod signed_area;
pub mod subdivide_segments;
pub mod sweep_event;

pub use helper::Float;

use self::connect_edges::connect_edges;
use self::fill_queue::fill_queue;
use self::subdivide_segments::subdivide;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operation {
    Intersection,
    Difference,
    Union,
    Xor,
}

pub trait BooleanOp<F, Rhs = Self>
where
    F: Float,
{
    fn boolean(&self, rhs: &Rhs, operation: Operation) -> MultiPolygon<F>;

    fn intersection(&self, rhs: &Rhs) -> MultiPolygon<F> {
        self.boolean(rhs, Operation::Intersection)
    }

    fn difference(&self, rhs: &Rhs) -> MultiPolygon<F> {
        self.boolean(rhs, Operation::Difference)
    }

    fn union(&self, rhs: &Rhs) -> MultiPolygon<F> {
        self.boolean(rhs, Operation::Union)
    }

    fn xor(&self, rhs: &Rhs) -> MultiPolygon<F> {
        self.boolean(rhs, Operation::Xor)
    }
}

impl<F> BooleanOp<F> for Polygon<F>
where
    F: Float,
{
    fn boolean(&self, rhs: &Polygon<F>, operation: Operation) -> MultiPolygon<F> {
        boolean_operation(&[self.clone()], &[rhs.clone()], operation)
    }
}

impl<F> BooleanOp<F, MultiPolygon<F>> for Polygon<F>
where
    F: Float,
{
    fn boolean(&self, rhs: &MultiPolygon<F>, operation: Operation) -> MultiPolygon<F> {
        boolean_operation(&[self.clone()], rhs.0.as_slice(), operation)
    }
}

impl<F> BooleanOp<F> for MultiPolygon<F>
where
    F: Float,
{
    fn boolean(&self, rhs: &MultiPolygon<F>, operation: Operation) -> MultiPolygon<F> {
        boolean_operation(self.0.as_slice(), rhs.0.as_slice(), operation)
    }
}

impl<F> BooleanOp<F, Polygon<F>> for MultiPolygon<F>
where
    F: Float,
{
    fn boolean(&self, rhs: &Polygon<F>, operation: Operation) -> MultiPolygon<F> {
        boolean_operation(self.0.as_slice(), &[rhs.clone()], operation)
    }
}

fn boolean_operation<F>(subject: &[Polygon<F>], clipping: &[Polygon<F>], operation: Operation) -> MultiPolygon<F>
where
    F: Float,
{
    let mut sbbox = BoundingBox {
        min: Coordinate {
            x: F::infinity(),
            y: F::infinity(),
        },
        max: Coordinate {
            x: F::neg_infinity(),
            y: F::neg_infinity(),
        },
    };
    let mut cbbox = sbbox;

    let mut event_queue = fill_queue(subject, clipping, &mut sbbox, &mut cbbox, operation);

    if sbbox.min.x > cbbox.max.x || cbbox.min.x > sbbox.max.x || sbbox.min.y > cbbox.max.y || cbbox.min.y > sbbox.max.y
    {
        return trivial_result(subject, clipping, operation);
    }

    let sorted_events = subdivide(&mut event_queue, &sbbox, &cbbox, operation);

    let contours = connect_edges(&sorted_events);

    // Convert contours into polygons
    let polygons: Vec<Polygon<F>> = contours
        .iter()
        .filter(|contour| contour.is_exterior())
        .map(|contour| {
            let exterior = LineString(contour.points.clone());
            let mut interios: Vec<LineString<F>> = Vec::new();
            for hole_id in &contour.hole_ids {
                interios.push(LineString(contours[*hole_id as usize].points.clone()));
            }
            Polygon::new(exterior, interios)
        })
        .collect();

    MultiPolygon(polygons)
}

fn trivial_result<F>(subject: &[Polygon<F>], clipping: &[Polygon<F>], operation: Operation) -> MultiPolygon<F>
where
    F: Float,
{
    match operation {
        Operation::Intersection => MultiPolygon(vec![]),
        Operation::Difference => MultiPolygon(Vec::from(subject)),
        Operation::Union | Operation::Xor => MultiPolygon(subject.iter().chain(clipping).cloned().collect()),
    }
}

/// A bounded 2D quadrilateral whose area is defined by minimum and maximum `Coordinates`.
///
/// A simple implementation copied from geo_types 0.4.0, because this version is a better
/// fit for the needs of this crate than the newer ones.
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoundingBox<T>
    where
        T: CoordinateType,
{
    pub min: Coordinate<T>,
    pub max: Coordinate<T>,
}

impl<T: CoordinateType> BoundingBox<T> {
    pub fn width(self) -> T {
        self.max.x - self.min.x
    }

    pub fn height(self) -> T {
        self.max.y - self.min.y
    }
}
