// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    //let y: Option<Point> = None;      This will print no match!

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
        //We can also modify _ by "None"" and if y will ever be equal to None it will panic
    }
    y; // Fix without deleting this line.
}
