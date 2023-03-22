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

fn left_shoot_growth(t: &Tree) -> (i16, i16) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    match thread_rng().gen_range(1..=10) {
        1|2 => y=-1,
        9|10 => y=1,
        _ => ()
    }
    match thread_rng().gen_range(1..=10) {
        1|2 => x = -2,
        3..=6 => x = -1,
        7..=9 => x = 0,
        _ => x = 1
    }
    return (x,y)
}

/// Same as trunk_growth
fn right_shoot_growth(t: &Tree) -> (i16, i16) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    match thread_rng().gen_range(1..=10) {
        1|2 => y=-1,
        9|10 => y=1,
        _ => ()
    }
    match thread_rng().gen_range(1..=10) {
        1|2 => x = 2,
        3..=6 => x = 1,
        7..=9 => x = 0,
        _ => x = -1
    }
    return (x,y)
}

const TRUNK_STRINGS: [&str;4] = ["/~","\\|","/|\\","|/"];
const SHOOT_STRINGS: [&str;6] = ["\\","\\_","\\|","/|","/","_/"];
fn choose_string(t: &Tree) -> &str {
    match t.state {
        TreeState::Trunk => {
            let d = trunk_growth(t);
            if (d.1 == 0) {return TRUNK_STRINGS[0]};
            match d.0 {
                n if n < 0 => return TRUNK_STRINGS[1],
                0 => return TRUNK_STRINGS[2],
                _ => return TRUNK_STRINGS[3]
            }
        },
        TreeState::BranchLeft => {
            let d = left_shoot_growth(t);
            if (d.1 > 0) {return SHOOT_STRINGS[0]}
            else if d.1 == 0 {return SHOOT_STRINGS[1]};
            match d.0 {
                n if n < 0 => return SHOOT_STRINGS[2],
                0 => SHOOT_STRINGS[3],
                _ => SHOOT_STRINGS[4]
            }
        },
        TreeState::BranchRight => {
            let d = right_shoot_growth(t);
            if (d.1 > 0) {return SHOOT_STRINGS[4]}
            else if d.1 == 0 {return SHOOT_STRINGS[5]};
            match d.0 {
                n if n < 0 => return SHOOT_STRINGS[2],
                0 => SHOOT_STRINGS[3],
                _ => SHOOT_STRINGS[4]
            }
        },
        _ => {return "shrimp"}
    }
}