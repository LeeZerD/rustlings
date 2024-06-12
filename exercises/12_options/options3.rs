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

    match &y { 
// de '&' car &y emprunte une référence à y, permettant de lire la valeur sans en prendre possession. Cela maintient y disponible après le match.
//match y sans '&'on prend possession de la valeur et y est déplacé dans match
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
