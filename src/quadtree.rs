//Failed attempt at trying to implement a quadtree for the game of life.
//I wasn't able to get it to work, so I'm just going to leave it here
//as a reference for future me.
//
//Pretty sure I am overcomplicating things, but I will leave this for future me to attempt.

use rand::Rng;

#[derive(Clone, Copy)]
struct Cell {
    posx: u32,
    posy: u32,
    is_alive: u8,
    alive_neighbors: u8,
}

impl Cell {
    fn new(posx: u32, posy: u32) -> Cell {
        Cell {
            posx,
            posy,
            is_alive: 0,
            alive_neighbors: 0,
        }
    }
}

struct Boundary {
    posx: u32,
    posy: u32,
    width: u32,
    height: u32,
}

impl Boundary {
    fn new(posx: u32, posy: u32, width: u32, height: u32) -> Boundary {
        Boundary {
            posx,
            posy,
            width,
            height,
        }
    }

    fn is_within(&self, cell: &Cell) -> bool {
        if self.posx <= cell.posx
            && cell.posx <= self.posx + self.width
            && self.posy <= cell.posy
            && cell.posy <= self.posy + self.height
        {
            return true;
        }
        false
    }
}

struct QuadTree {
    bounds: Boundary,
    capacity: u32,
    cells: Vec<Cell>,
    children: Vec<QuadTree>,
    is_leaf: bool,
}

impl QuadTree {
    fn new(bounds: Boundary) -> QuadTree {
        QuadTree {
            bounds,
            capacity: 4,
            cells: vec![],
            children: vec![],
            is_leaf: true,
        }
    }

    fn insert(&mut self, cell: &Cell) {
        println!("Trying toInserting cell at {}, {}", cell.posx, cell.posy);
        if !self.bounds.is_within(&cell) {
            return;
        }

        if self.is_leaf && (self.cells.len() as u32) < self.capacity {
            self.cells.push(*cell);
        } else {
            if self.is_leaf {
                self.split();
            }
            self.children[0].insert(cell);
            self.children[1].insert(cell);
            self.children[2].insert(cell);
            self.children[3].insert(cell);
        }
    }

    fn split(&mut self) {
        println!("Subdividing");
        println!("Creating child 1");

        self.children.push(QuadTree::new(Boundary::new(
            self.bounds.posx,
            self.bounds.posy,
            self.bounds.width / 2,
            self.bounds.height / 2,
        )));

        println!("Creating child 2");
        self.children.push(QuadTree::new(Boundary::new(
            self.bounds.posx + self.bounds.width / 2,
            self.bounds.posy,
            self.bounds.width / 2,
            self.bounds.height / 2,
        )));

        println!("Creating child 3");
        self.children.push(QuadTree::new(Boundary::new(
            self.bounds.posx,
            self.bounds.posy + self.bounds.height / 2,
            self.bounds.width / 2,
            self.bounds.height / 2,
        )));

        println!("Creating child 4");
        self.children.push(QuadTree::new(Boundary::new(
            self.bounds.posx + self.bounds.width / 2,
            self.bounds.posy + self.bounds.height / 2,
            self.bounds.width / 2,
            self.bounds.height / 2,
        )));
        self.is_leaf = false;
    }
}

fn main() {
    println!("Starting game of life");

    let boundary = Boundary::new(0, 0, GRID_WIDTH, GRID_HEIGHT);
    let mut tree = QuadTree::new(boundary);

    for _ in 0..64 {
        let cell = Cell::new(
            rand::thread_rng().gen_range(0..GRID_WIDTH),
            rand::thread_rng().gen_range(0..GRID_HEIGHT),
        );

        tree.insert(&cell);
    }
}
