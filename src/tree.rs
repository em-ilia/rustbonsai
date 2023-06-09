use crossterm::style::{StyledContent, Stylize};
use rand::{thread_rng, Rng};

// Growth Constants
const EDGE_PENALTY: (i16, i16, i16, i16) = (-2, 3, 4, -4); // How strongly to fear the edge
const KNOT_RATIO: u32 = 22; // Decrease for more knots
const KNOT_AGE: i16 = 20; // Minimum age to knot
const TRANSITION_RATIO: u32 = 20; // Decrease for earlier sideways branching
const TRANSITION_AGE: i16 = 30; // Minimum age to branch
const TRANSITION_PENALTY: i16 = 20; // How much age to add when branching
const LEAF_AGE: i16 = 60; // When we should start generating leaves
const DEATH_AGE: i16 = LEAF_AGE + 30; // When to die :(

// const INITIAL_LIFE: i16 = 32;
pub struct Tree {
    x: i16,
    y: i16,
    dxy: (i16, i16),
    pub age: i16,
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
            y: 4,
            dxy: (0, 0),
            age: 0,
            state: TreeState::Trunk,
            knots: Vec::new(),
            xmax,
            ymax,
        }
    }
    fn new_at(&self, state: TreeState) -> Self {
        Tree {
            x: self.x,
            y: self.y,
            dxy: self.dxy,
            age: self.age,
            state, // Should have RNG here instead
            knots: Vec::new(),
            xmax: self.xmax,
            ymax: self.ymax,
        }
    }

    /// Output is top, bottom, left, right
    fn check_boundary(&self) -> (i16, i16, i16, i16) {
        return (
            self.y - self.ymax,
            self.y - 4,
            self.x + self.xmax,
            self.x - self.xmax,
        );
    }

    pub fn grow(&mut self) {
        // Grow all children
        for tree in &mut self.knots {
            tree.grow()
        }
        self.age += 1;

        // Handle old and dead trees
        if self.age > DEATH_AGE || self.state == TreeState::Dead {
            self.state = TreeState::Dead;
            return ();
        };

        if self.age > LEAF_AGE {
            self.state = TreeState::Leaves;
        }

        self.dxy = match self.state {
            TreeState::Trunk => trunk_growth(self),
            TreeState::BranchLeft => left_shoot_growth(self),
            TreeState::BranchRight => right_shoot_growth(self),
            TreeState::Dead => (0, 0),
            TreeState::Leaves => leaf_growth(self), // Not sure what to do here
        };
        self.x += self.dxy.0;
        self.y += self.dxy.1;

        // Correct out of bounds
        let bnd = self.check_boundary();
        if bnd.0 > -2 {
            self.y += EDGE_PENALTY.0
        }
        if bnd.1 < 0 {
            self.y += EDGE_PENALTY.1
        }
        if bnd.2 < 5 {
            self.x += EDGE_PENALTY.2
        }
        if bnd.3 > -5 {
            self.x += EDGE_PENALTY.3
        }

        // State transitions
        if self.state == TreeState::Trunk
            && self.age > TRANSITION_AGE
            && thread_rng().gen_ratio(1, TRANSITION_RATIO)
        {
            self.age += TRANSITION_PENALTY;
            self.state = if thread_rng().gen_bool(0.8) {
                // Tree seems to deviate right,
                // so branching left more often
                // adds nice balance
                TreeState::BranchLeft
            } else {
                TreeState::BranchRight
            };
        }

        // Occasionally create a knot
        if thread_rng().gen_ratio(1, KNOT_RATIO) && self.age > KNOT_AGE {
            self.force_knot();
        }
    }

    pub fn observe(&self) -> Vec<(i16, i16, StyledContent<&str>)> {
        let mut res: Vec<(i16, i16, StyledContent<&str>)> = Vec::new();
        res.push((self.x, self.y, choose_string(self)));
        for tree in &self.knots {
            res.append(&mut tree.observe());
        }

        return res;
    }

    /// Return true only if every tree is dead
    pub fn is_dead(&self) -> bool {
        if self.state != TreeState::Dead {
            return false;
        }
        for tree in &self.knots {
            if !tree.is_dead() {
                return false;
            }
        }
        return true;
    }

    pub fn force_knot(&mut self) {
        match self.state {
            TreeState::BranchLeft => self.knots.push(self.new_at(TreeState::Trunk)),
            TreeState::BranchRight => self.knots.push(self.new_at(TreeState::Trunk)),
            TreeState::Trunk => self.knots.push(self.new_at(if thread_rng().gen_bool(0.5) {
                TreeState::BranchRight
            } else {
                TreeState::BranchLeft
            })),
            _ => (),
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
        0..=10 => return (thread_rng().gen_range(-1..=1), 0),
        11..=TRANSITION_AGE => {
            let x = match thread_rng().gen_range(1..=10) {
                1 | 2 => -1,
                3 | 4 => 1,
                _ => 0,
            };
            let y = if t.age % 6 == 0 { 1 } else { 0 };
            return (x, y);
        }
        _ => {
            let x = thread_rng().gen_range(-1..=2);
            let y = if thread_rng().gen_ratio(1, 3) { 1 } else { 0 };
            return (x, y);
        }
    }
}

fn left_shoot_growth(_t: &Tree) -> (i16, i16) {
    let x: i16;
    let mut y: i16 = 0;
    match thread_rng().gen_range(1..=10) {
        1 => y = -1,
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

/// Same as left_shoot_growth
fn right_shoot_growth(t: &Tree) -> (i16, i16) {
    let xy = left_shoot_growth(t);
    return (-xy.0, xy.1);
}

fn leaf_growth(_t: &Tree) -> (i16, i16) {
    let y: i16 = match thread_rng().gen_range(1..=15) {
        // Leaves should not be vertical!!
        1 => -1,
        2 => 1,
        _ => 0,
    };
    let x: i16 = thread_rng().gen_range(-1..=1);
    return (x, y);
}

const TRUNK_STRINGS: [&str; 4] = ["/~", "\\|", "/|\\", "|/"];
const SHOOT_STRINGS: [&str; 6] = ["\\", "\\_", "\\|", "/|", "/", "_/"];
fn choose_string(t: &Tree) -> StyledContent<&str> {
    let s: StyledContent<&str>;
    match t.state {
        TreeState::Trunk => {
            if t.dxy.1 == 0 {
                s = TRUNK_STRINGS[0].dark_yellow();
            } else {
                match t.dxy.0 {
                    n if n < 0 => s = TRUNK_STRINGS[1].dark_yellow(),
                    0 => s = TRUNK_STRINGS[2].dark_yellow(),
                    _ => s = TRUNK_STRINGS[3].dark_yellow(),
                }
            }
        }
        TreeState::BranchLeft => {
            if t.dxy.1 > 0 {
                SHOOT_STRINGS[0].dark_yellow();
            } else if t.dxy.1 == 0 {
                SHOOT_STRINGS[1].dark_yellow();
            };
            match t.dxy.0 {
                n if n < 0 => s = SHOOT_STRINGS[2].dark_yellow(),
                0 => s = SHOOT_STRINGS[3].dark_yellow(),
                _ => s = SHOOT_STRINGS[4].dark_yellow(),
            }
        }
        TreeState::BranchRight => {
            if t.dxy.1 > 0 {
                SHOOT_STRINGS[4].dark_yellow();
            } else if t.dxy.1 == 0 {
                SHOOT_STRINGS[5].dark_yellow();
            };
            match t.dxy.0 {
                n if n < 0 => s = SHOOT_STRINGS[2].dark_yellow(),
                0 => s = SHOOT_STRINGS[3].dark_yellow(),
                _ => s = SHOOT_STRINGS[4].dark_yellow(),
            }
        }
        TreeState::Leaves => s = "&&&".green(),
        _ => s = "&".yellow(),
    };

    match t.state {
        TreeState::Trunk | TreeState::BranchLeft | TreeState::BranchRight => {
            if thread_rng().gen_ratio(1, 3) {
                s.bold()
            } else {
                s
            }
        }
        TreeState::Leaves => match thread_rng().gen_range(1..=10) {
            1 | 2 => s.bold(),
            3 => s.bold().dark_green(),
            4 | 5 => s.dark_green(),
            _ => s,
        },
        _ => {
            if thread_rng().gen_ratio(1, 5) {
                s.bold()
            } else {
                s
            }
        }
    }
}
