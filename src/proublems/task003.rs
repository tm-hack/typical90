#![allow(non_snake_case)]

use proconio::*;
use std::collections::BinaryHeap;

struct Edge {
    to: usize,
    cost: i32,
}

struct Graph {
    N: usize,
    edge_data: Vec<Vec<Edge>>,
    dist_data: Vec<i32>,
    diameter: i32,
    endpoint: (usize, usize),
}

impl Graph {
    const INF: i32 = 1_000_000_000;

    fn new(N: usize, edges: Vec<(usize, usize)>) -> Graph {
        let mut edge_data: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..N {
            edge_data.push(Vec::new());
        }
        let mut graph = Graph {
            N,
            edge_data,
            dist_data: vec![0; N],
            endpoint: (0, 0),
            diameter: 0,
        };

        for edge in edges.iter() {
            graph.add_edge(edge.0 - 1, edge.1 - 1, 1);
        }

        graph
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: i32) {
        self.edge_data[u].push(Edge { to: v, cost });
        self.edge_data[v].push(Edge { to: u, cost });
    }

    fn dijkstra(&mut self, s: usize) {
        for i in 0..self.N {
            self.dist_data[i] = Graph::INF;
        }
        let mut binary_heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        self.dist_data[s] = 0;
        binary_heap.push((0, s));

        while !binary_heap.is_empty() {
            let (dist, v) = binary_heap.pop().unwrap();
            if self.dist_data[v] < dist {
                continue;
            }
            for edge in self.edge_data[v].iter() {
                if self.dist_data[edge.to] > self.dist_data[v] + edge.cost {
                    self.dist_data[edge.to] = self.dist_data[v] + edge.cost;
                    binary_heap.push((self.dist_data[v] + edge.cost, edge.to));
                }
            }
        }
    }

    fn calc_diameter(&mut self) {
        let mut endpoint_i = 0;
        let mut endpoint_j = 0;
        let mut diameter = 0;

        self.dijkstra(0);
        for (i, val) in self.dist_data.iter().enumerate() {
            if diameter < *val {
                endpoint_i = i;
                diameter = *val;
            }
        }

        self.dijkstra(endpoint_i);
        for (i, val) in self.dist_data.iter().enumerate() {
            if diameter < *val {
                endpoint_j = i;
                diameter = *val;
            }
        }

        self.endpoint = (endpoint_i, endpoint_j);
        self.diameter = diameter;
    }
}

pub fn main() {
    input! {
        N: usize,
        edges: [(usize, usize); N-1],
    }

    let mut graph = Graph::new(N, edges);
    graph.calc_diameter();

    println!("{}", graph.diameter + 1);
}
