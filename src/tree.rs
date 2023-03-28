use rand::{thread_rng, Rng};

// const INITIAL_LIFE: i16 = 32;
pub struct Tree {
    x: i16,
    y: i16,
    age: i16,
    pub state: TreeState,
    knots: Vec<Tree>,
    xmax: i16,
    ymax: i16,
}
#[derive(PartialEq, Debug)]
pub enum TreeState {
    Trunk,
    BranchLeft,
    BranchRight,
    Leaves,
    Dead,
}
impl Tree {
    pub fn new(xmax: i16, ymax: i16) -> Self {
        Tree {
            x: 0,
            y: 0,
            age: 0,
            state: TreeState::Trunk,
            knots: Vec::new(),
            xmax,
            ymax,
        }
    }
    fn new_at(&self) -> Self {
        Tree {
            x: self.x,
            y: self.y,
            age: self.age,
            state: TreeState::Trunk, // Should have RNG here
            knots: Vec::new(),
            xmax: self.xmax,
            ymax: self.ymax,
        }
    }

    /// Output is top, bottom, left, right
    fn check_boundary(&self) -> (i16, i16, i16, i16) {
        return (self.ymax - self.y,
                self.y,
                self.x + self.xmax,
                self.x - self.xmax)
    }

    pub fn grow(&mut self) {
        self.age += 1;

        // Handle old and dead trees
        if self.age > 100 || self.state == TreeState::Dead {
            self.state = TreeState::Dead;
            return ();
        };

        let d = match self.state {
            TreeState::Trunk => trunk_growth(self),
            TreeState::BranchLeft => left_shoot_growth(self),
            TreeState::BranchRight => right_shoot_growth(self),
            TreeState::Dead => (0, 0),
            TreeState::Leaves => (0, 0), // Not sure what to do here
        };
        self.x += d.0;
        self.y += d.1;

        // State transitions
        if self.state == TreeState::Trunk && thread_rng().gen_ratio(1, 15) {
            self.state = if thread_rng().gen_bool(0.5) {
                TreeState::BranchLeft
            } else {
                TreeState::BranchRight
            };
        }

        // Occasionally create a knot
        if thread_rng().gen_ratio(1, 30) {
            self.knots.push(self.new_at());
        }

        // Grow all children
        for tree in &mut self.knots {
            tree.grow()
        }
    }

    pub fn observe(&self) -> Vec<(i16, i16, &str)> {
        let mut res: Vec<(i16, i16, &str)> = Vec::new();
        res.push((self.x, self.y, choose_string(self)));
        for tree in &self.knots {
            res.append(&mut tree.observe());
        }

        return res;
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
            let y = if t.age % 3 == 0 { 1 } else { 0 };
            return (x, y);
        }
        _ => {
            let x = thread_rng().gen_range(-1..=2);
            let y = if thread_rng().gen_ratio(1, 5) { 1 } else { 0 };
            return (x, y);
        }
    }
}

/// Same as trunk_growth
fn left_shoot_growth(_t: &Tree) -> (i16, i16) {
    let x: i16;
    let mut y: i16 = 0;
    match thread_rng().gen_range(1..=10) {
        1 | 2 => y = -1,
        9 | 10 => y = 1,
        _ => (),
    }
    match thread_rng().gen_range(1..=10) {
        1 | 2 => x = -2,
        3..=6 => x = -1,
        7..=9 => x = 0,
        _ => x = 1,
    }
    return (x, y);
}

/// Same as trunk_growth
fn right_shoot_growth(_t: &Tree) -> (i16, i16) {
    let x: i16;
    let mut y: i16 = 0;
    match thread_rng().gen_range(1..=10) {
        1 | 2 => y = -1,
        9 | 10 => y = 1,
        _ => (),
    }
    match thread_rng().gen_range(1..=10) {
        1 | 2 => x = 2,
        3..=6 => x = 1,
        7..=9 => x = 0,
        _ => x = -1,
    }
    return (x, y);
}

const TRUNK_STRINGS: [&str; 4] = ["/~", "\\|", "/|\\", "|/"];
const SHOOT_STRINGS: [&str; 6] = ["\\", "\\_", "\\|", "/|", "/", "_/"];
fn choose_string(t: &Tree) -> &str {
    match t.state {
        TreeState::Trunk => {
            let d = trunk_growth(t);
            if d.1 == 0 {
                return TRUNK_STRINGS[0];
            };
            match d.0 {
                n if n < 0 => return TRUNK_STRINGS[1],
                0 => return TRUNK_STRINGS[2],
                _ => return TRUNK_STRINGS[3],
            }
        }
        TreeState::BranchLeft => {
            let d = left_shoot_growth(t);
            if d.1 > 0 {
                return SHOOT_STRINGS[0];
            } else if d.1 == 0 {
                return SHOOT_STRINGS[1];
            };
            match d.0 {
                n if n < 0 => return SHOOT_STRINGS[2],
                0 => SHOOT_STRINGS[3],
                _ => SHOOT_STRINGS[4],
            }
        }
        TreeState::BranchRight => {
            let d = right_shoot_growth(t);
            if d.1 > 0 {
                return SHOOT_STRINGS[4];
            } else if d.1 == 0 {
                return SHOOT_STRINGS[5];
            };
            match d.0 {
                n if n < 0 => return SHOOT_STRINGS[2],
                0 => SHOOT_STRINGS[3],
                _ => SHOOT_STRINGS[4],
            }
        }
        _ => return "shrimp",
    }
}
