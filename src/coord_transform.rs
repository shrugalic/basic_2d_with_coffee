use coffee::graphics::{Point, Rectangle, Shape};
use std::ops::RangeInclusive;

#[derive(Copy, Clone)]
pub(crate) struct Coord2 {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CoordinateTransformation {
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
    tile_size: f32,
}
impl CoordinateTransformation {
    pub(crate) fn from_points(width: f32, height: f32, points: &[Coord2]) -> Self {
        let min_x = points.iter().map(|p| p.x).min().unwrap() as f32;
        let max_x = points.iter().map(|p| p.x).max().unwrap() as f32;
        let min_y = points.iter().map(|p| p.y).min().unwrap() as f32;
        let max_y = points.iter().map(|p| p.y).max().unwrap() as f32;
        let xr = min_x..=max_x;
        let yr = min_y..=max_y;

        let w_count = (max_x - min_x) + 1.0;
        let h_count = (max_y - min_y) + 1.0;
        let ppp = f32::min(width / w_count, height / h_count); // pixels-per-point

        /*
        println!(
            "Board: w: {}, h: {}, x: {} to {}, y: {} to {}, ppp: {}",
            w_count,
            h_count,
            xr.start(),
            xr.end(),
            yr.start(),
            yr.end(),
            ppp
        );
        */

        CoordinateTransformation {
            x_range: xr,
            y_range: yr,
            tile_size: ppp,
        }
    }
    pub(crate) fn square_at(&self, pos: &Coord2) -> Shape {
        let top_left = self.point_at(pos);
        Shape::Rectangle(Rectangle {
            x: top_left.x,
            y: top_left.y,
            width: self.tile_size,
            height: self.tile_size,
        })
    }
    pub(crate) fn point_at(&self, pos: &Coord2) -> Point {
        Point::new(
            self.tile_size * (pos.x as f32 - *self.x_range.start()),
            self.tile_size * (pos.y as f32 - *self.y_range.start()),
        )
    }
}
