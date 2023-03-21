enum Tree {
    Branch(Branch),
    Leaf(Leaf)
}

pub struct Trunk {
    x_range: f64,
    y_range: f64,
    child: Tree
}
impl Trunk {
    fn new(radius_x: f64, radius_y: f64) -> Self {
        Trunk {
            x_range: radius_x,
            y_range: radius_y,
            child: Tree::Leaf(Leaf { x: 0.0, y: 0.0 })
            }
    }
}

struct Branch {
    x: f64,
    y: f64,
    dx: f64,
    m: i8, // We're only going to permit a few slopes
    child: Box<Tree>
}

struct Leaf {
    x: f64,
    y: f64
}