fn main() {
    let points: Vec<Point<f64>> = Vec::from([
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 9020.0 },
        Point { x: 1.0, y: 4845.0 },
        Point { x: 2.0, y: 6189.0 },
        Point { x: 3.0, y: 9190.0 },
        Point { x: 4.0, y: 9471.0 },
        Point { x: 5.0, y: 9313.0 },
        Point { x: 6.0, y: 504.0 },
        Point { x: 7.0, y: 4555.0 },
        Point { x: 8.0, y: 3455.0 },
    ]);
    let size = Point { x: 150.0, y: 100.0 };
    println!(
        "{}",
        generate_svg(&truncate_chart(&points, size), size, String::from("red"))
    );
}
