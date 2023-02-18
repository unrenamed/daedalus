use crate::app::{
    grid::cell::Cell,
    state::MazeSnapshot,
    utils::{
        arena::{ArenaTree, NodeId},
        types::Pos,
    },
};
use rand::prelude::*;

use super::{Generator, IGenerator, Snapshot};

type Edge = (usize, usize, Cell);
type Edges = Vec<Edge>;

pub struct Kruskal {
    generator: Generator,
}

impl Kruskal {
    fn populate_arena_tree(&self) -> ArenaTree {
        let mut arena = ArenaTree::new();
        for _ in 0..self.generator.grid.width() * self.generator.grid.height() {
            arena.new_node();
        }
        arena
    }

    fn populate_edges(&self) -> Edges {
        let mut edges: Edges = vec![];
        for y in 0..self.generator.grid.height() {
            for x in 0..self.generator.grid.width() {
                if y > 0 {
                    edges.push((x, y, Cell::NORTH))
                }
                if x > 0 {
                    edges.push((x, y, Cell::WEST))
                }
            }
        }
        edges
    }
}

impl IGenerator for Kruskal {
    fn init(width: usize, height: usize) -> Self {
        let generator = Generator::new(width, height);
        Self { generator }
    }

    fn run(&mut self) -> Vec<MazeSnapshot> {
        let mut arena = self.populate_arena_tree();
        let mut edges: Edges = self.populate_edges();

        edges.shuffle(&mut thread_rng());
        while edges.len() > 0 {
            let edge: Option<Edge> = edges.pop();
            if edge.is_none() {
                break;
            }

            let (x, y, dir) = edge.unwrap();

            let (nx, ny) = match self.generator.grid.get_next_cell_pos((x, y), dir) {
                Ok(next) => next,
                Err(_) => continue,
            };

            let node1 = NodeId(y * self.generator.grid.width() + x);
            let node2 = NodeId(ny * self.generator.grid.width() + nx);

            if !arena.connected(node1, node2) {
                self.generator.highlights.clear();
                self.generator.highlights.push((x, y) as Pos);
                self.generator.highlights.push((nx, ny) as Pos);
                self.generator.make_snapshot();

                arena.connect(node1, node2);
                self.generator.grid.carve_passage((x, y), dir).unwrap();
            }
        }

        self.generator.highlights.clear();
        self.generator.make_snapshot();

        self.generator.get_snapshots()
    }
}
