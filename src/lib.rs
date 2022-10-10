use std::cmp::PartialOrd;
use std::fmt::Display;
use std::ops::{Add, Div, Sub};
use std::vec::Vec;

#[derive(Clone, Copy)]
struct Point<T: Copy + Display + PartialOrd + Div<Output = T>> {
    x: T,
    y: T,
}

// assumes x points are sorted
fn truncate_chart<T: Copy + Display + PartialOrd + Div<Output = T>>(
    points: &Vec<Point<T>>,
    size: Point<T>,
) -> Vec<Point<T>> {
    let (mut min_y, mut max_y) = (points[0].y, points[0].y);
    let max_x = points[points.len() - 1].x;

    for p in points {
        if p.y > max_y {
            max_y = p.y;
        }
        if p.y < min_y {
            min_y = p.y;
        }
    }

    let mut adjusted_points: Vec<Point<T>> = Vec::new();
    for p in points {
        let n = Point {
            x: p.x / (max_x / size.x),
            y: p.y / (max_y / size.y),
        };
        adjusted_points.push(n);
    }

    return adjusted_points;
}

fn generate_svg<T: Copy + Display + PartialOrd + Sub<Output = T> + Div<Output = T>>(
    points: &Vec<Point<T>>,
    size: Point<T>,
    color: String,
) -> String {
    let mut svg : String = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewbox=\"0 0 {} {}\">", size.x, size.y, size.x, size.y);

    svg += "<polyline fill=\"none\"";
    svg += " stroke=\"";
    svg += &color;
    svg += "\"";
    svg += " points=\"";

    for p in points {
        let point_str = format!("{0:.2},{1:.2} ", p.x, size.y - p.y);
        svg.push_str(&point_str);
    }

    svg += "\"/></svg>";

    return svg;
}