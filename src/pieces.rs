use std::f32::consts::PI;

use iced::widget::canvas::Path;
use iced::Point;

pub fn draw_stones(stones: u8, center: Point, hexagon_radius: f32) -> Vec<Path> {
    let radius = hexagon_radius / 8.0;
    let mut paths = Vec::new();

    match stones {
        1 => paths.push(draw_circles(center.x, center.y, radius)),
        2 => {
            paths.push(draw_circles(
                center.x - hexagon_radius / 4.0,
                center.y,
                radius,
            ));
            paths.push(draw_circles(
                center.x + hexagon_radius / 4.0,
                center.y,
                radius,
            ));
        }
        3 => {
            paths.push(draw_circles(
                center.x,
                center.y - hexagon_radius / 4.0,
                radius,
            ));
            paths.push(draw_circles(
                center.x + (hexagon_radius / 4.0) * (3.0f32.sqrt() / 2.0),
                center.y + hexagon_radius / 8.0,
                radius,
            ));
            paths.push(draw_circles(
                center.x - (hexagon_radius / 4.0) * (3.0f32.sqrt() / 2.0),
                center.y + hexagon_radius / 8.0,
                radius,
            ));
        }
        4 => {
            paths.push(draw_circles(
                center.x - hexagon_radius / 4.0,
                center.y - hexagon_radius / 4.0,
                radius,
            ));
            paths.push(draw_circles(
                center.x + hexagon_radius / 4.0,
                center.y - hexagon_radius / 4.0,
                radius,
            ));
            paths.push(draw_circles(
                center.x - hexagon_radius / 4.0,
                center.y + hexagon_radius / 4.0,
                radius,
            ));
            paths.push(draw_circles(
                center.x + hexagon_radius / 4.0,
                center.y + hexagon_radius / 4.0,
                radius,
            ));
        }
        5 => {
            paths.push(draw_circles(
                center.x - hexagon_radius / 4.0,
                center.y - hexagon_radius / 4.0,
                radius,
            ));
            paths.push(draw_circles(
                center.x + hexagon_radius / 4.0,
                center.y - hexagon_radius / 4.0,
                radius,
            ));
            paths.push(draw_circles(
                center.x - hexagon_radius / 4.0,
                center.y + hexagon_radius / 4.0,
                radius,
            ));
            paths.push(draw_circles(
                center.x + hexagon_radius / 4.0,
                center.y + hexagon_radius / 4.0,
                radius,
            ));
            paths.push(draw_circles(center.x, center.y, radius));
        }
        6 => {
            paths.push(draw_circles(
                center.x + hexagon_radius / 3.0,
                center.y,
                radius,
            ));
            paths.push(draw_circles(
                center.x + (hexagon_radius / 3.0) * (PI / 3.0).cos(),
                center.y + (hexagon_radius / 3.0) * (PI / 3.0).sin(),
                radius,
            ));
            paths.push(draw_circles(
                center.x + (hexagon_radius / 3.0) * (2.0 * PI / 3.0).cos(),
                center.y + (hexagon_radius / 3.0) * (2.0 * PI / 3.0).sin(),
                radius,
            ));
            paths.push(draw_circles(
                center.x + (hexagon_radius / 3.0) * (3.0 * PI / 3.0).cos(),
                center.y + (hexagon_radius / 3.0) * (3.0 * PI / 3.0).sin(),
                radius,
            ));
            paths.push(draw_circles(
                center.x + (hexagon_radius / 3.0) * (4.0 * PI / 3.0).cos(),
                center.y + (hexagon_radius / 3.0) * (4.0 * PI / 3.0).sin(),
                radius,
            ));
            paths.push(draw_circles(
                center.x + (hexagon_radius / 3.0) * (5.0 * PI / 3.0).cos(),
                center.y + (hexagon_radius / 3.0) * (5.0 * PI / 3.0).sin(),
                radius,
            ));
        }

        _ => paths.push(draw_circles(center.x, center.y, radius)),
    }
    paths
}

fn draw_circles(center_x: f32, center_y: f32, radius: f32) -> Path {
    Path::circle(Point::new(center_x, center_y), radius)
}
