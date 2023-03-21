enum Tree {
    Branch(Branch),
    Leaf(Leaf)
}

struct Tree {
    x: f64,
    y: f64,
    age: i16,
    state: TreeState,
    knots: Vec<Tree>
}
enum TreeState {
    Trunk,
    BranchLeft,
    BranchRight,
    Leaves,
    Dead
}
impl Tree {
    fn new() -> Self {
        Tree {
            x: 0.0,
            y: 0.0,
            age: 0,
            state: TreeState::Trunk,
            knots: Vec::new()
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