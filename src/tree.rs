use rand::{thread_rng, Rng};

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

/// trunk_growth takes a tree in
/// a trunk state and returns:
/// * a dx integer, the amount to grow 
/// UNTESTED
fn trunk_growth(t: &Tree) -> (i16, i16) {
    // New trunk:
    match t.age {
        // New trunks
        0..=3 => return (thread_rng().gen_range(0..=2), 0),
        4..=15 => {
            let x = thread_rng().gen_range(-1..=2);
            let y = if (t.age % 3 == 0) {1} else {0};
            return (x,y);
        }
        _ => {
            let x = thread_rng().gen_range(-1..=2);
            let y = if thread_rng().gen_ratio(1, 5)
                            {1} else {0};
            return (x,y);
        }
    }
}