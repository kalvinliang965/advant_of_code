use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::mem::swap;
use std::cmp::Reverse;


// this is dangerous when running test cases.....
static COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Point {
    val: u64,
    x: u64,
    y: u64,
    z: u64
}

impl Point {
    fn new(x: u64, y: u64, z: u64) -> Self {
        let val = COUNTER.fetch_add(1, Ordering::Relaxed);
        // println!("{}", val);
        Self {val, x, y, z}
    }
}

// use this function to avoid global counter
struct PointFactory {
    counter: u64,
}

impl PointFactory {
    fn new() -> Self {
        Self { counter: 0 }
    }
    fn new_point(&mut self, x: u64, y: u64, z: u64) -> Point {
        let val = self.counter;
        self.counter += 1;
        // println!("{}", val);
        Point { val, x, y, z}
    }
}

#[derive(Debug)]
pub enum FileError {
    OpenFailed,
    ReadFailed,
    WriteFailed,
    InvalidFormat,
}

// assign each junction box a unique id, val
// return a map containing neighbor of each point
pub fn read_input(filepath: &str) -> Result<Vec<Point>, FileError> {
    let file = File::open(filepath).map_err(|_| FileError::OpenFailed)?;
    let reader = BufReader::new(file);
    let mut pf = PointFactory::new();
    reader
        .lines()
        .map(|line| {
            let line = line.map_err(|_| FileError::ReadFailed)?;
            let mut parts = line.split(",");
            let a = parts
                    .next()
                    .ok_or(FileError::InvalidFormat)?
                    .parse()
                    .map_err(|_| FileError::InvalidFormat)?;

            let b = parts
                    .next()
                    .ok_or(FileError::InvalidFormat)?
                    .parse()
                    .map_err(|_| FileError::InvalidFormat)?;

            let c = parts
                    .next()
                    .ok_or(FileError::InvalidFormat)?
                    .parse()
                    .map_err(|_| FileError::InvalidFormat)?;

            Ok(pf.new_point(a, b, c))
        })
        .collect()
}


// if A > B then A^2 > B^2
fn get_dist2(p: &Point, q: &Point) -> u64 {
    let dx = p.x as i64 - q.x as i64;
    let dy = p.y as i64 - q.y as i64;
    let dz = p.z as i64 - q.z as i64;
    (dx*dx + dy*dy + dz*dz) as u64
}

fn get_edges(v: &Vec<Point>) -> Vec<(u64, u64, u64)> {
    let mut res = Vec::new();
    let n = v.len();
    for i in 0..n {
        for j in i+1..n {
            let p = &v[i];
            let q = &v[j];
            let d = get_dist2(p, q);
            res.push((p.val, d, q.val));
        }
    }
    res
}

struct DSU {
    Parent: Vec<u64>,
    Size: Vec<u64>,
}

impl DSU {
    fn new(n: u64) -> Self {
        let mut Parent = Vec::new();
        let mut Size = Vec::new();
        for i in 0..n {
            Parent.push(i); // parent is them self
            Size.push(1);
        }
        Self {
            Parent,
            Size,
        }
    }
    fn Find(&mut self, mut node: u64) -> u64 {
        // println!("{}", node);
        let mut root = self.Parent[node as usize];
        while self.Parent[root as usize] != root {
            root = self.Parent[root  as usize];
        }
        while self.Parent[node as usize] != node {
            let next = self.Parent[node as usize];
            self.Parent[node as usize] = root;
            node = next;
        }
        root
    }
    fn Unify(&mut self, p: u64, q: u64) -> bool {
        let mut pr = self.Find(p);
        let mut qr = self.Find(q);
        if pr == qr {
            return false;
        }
        // make sure pr size is the largest
        if self.Size[pr as usize] < self.Size[qr as usize] {
            swap(&mut pr, &mut qr);
        }
        self.Parent[qr as usize] = pr;
        self.Size[pr as usize] += self.Size[qr as usize];
        self.Size[qr as usize] = 1;
        true
    }
}

fn node_count_from_edges(m: u64) -> Option<u64> {
    let disc = 1 + 8 * m;
    let sqrt = (disc as f64).sqrt();
    if (sqrt.round() as u64).pow(2) != disc {
        return None; // not valid complte graph
    }
    let n = (1.0 + sqrt) / 2.0;
    Some(n as u64)
}

pub fn solve1(v: &Vec<Point>, k: u64) -> u64 {
    let mut res = 1;
    let edges = get_edges(v);
    let n = node_count_from_edges(edges.len() as u64).expect("not complete graph: error in get_edges");
    let mut heap = BinaryHeap::new();
    let mut dsu = DSU::new(n);
    // println!("{}", edges.len());
    // assert!(k < n);
    for (p, w, q) in edges {
        heap.push(Reverse((w, p, q)));
    }
    for i in 0..k {
        if let Some(Reverse((_, p, q))) = heap.pop() {
            dsu.Unify(p, q);
        }
    }
    let mut vis = HashSet::new();
    for i in 0..n {
        let parent = dsu.Parent[i as usize];
        if !vis.contains(&parent) {
            vis.insert(parent);
        }
    }

    let mut v = dsu.Size.clone();
    v.sort();
    
    if v.len() < 3 {
        panic!("Need at least 3 elements");
    }

    let n = v.len();
    let a = v[n - 1];
    let b = v[n - 2];
    let c = v[n - 3];
    let res = a.checked_mul(b)
           .and_then(|x| x.checked_mul(c))
           .expect("Overflow while multiplying the three largest values");
    res
}

// connect all of themm untile they are in same circuit -> MST
// get the product of the last k node
pub fn solve2(v: &Vec<Point>, k: u64) -> u64 {
    let mut res = 1u64;

    // val -> x mapping
    let get_x: HashMap<u64, u64> = v
        .iter()
        .map(|p| (p.val, p.x))
        .collect();

    // build edges
    let mut edges = get_edges(v);
    let n = v.len() as u64;

    // sort edges by weight
    edges.sort_unstable_by_key(|e| e.1);

    let mut dsu = DSU::new(n);

    let mut merges = Vec::new();

    // Kruskal
    for &(p, _, q) in &edges {
        if dsu.Unify(p, q) {
            merges.push((p, q));
        }
    }

    // MST should have n-1 merges
    assert_eq!(merges.len() as u64, n - 1);

    // take the LAST k merges
    let last_k = &merges[merges.len() - k as usize ..];

    for &(p, q) in last_k {
        let a = get_x[&p];
        let b = get_x[&q];
        res *= a * b;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_read_input() {
        let exp = vec![
            Point::new(162,817,812),
            Point::new(57,618,57),
            Point::new(906,360,560),
            Point::new(592,479,940),
            Point::new(352,342,300),
            Point::new(466,668,158),
            Point::new(542,29,236),
            Point::new(431,825,988),
            Point::new(739,650,466),
            Point::new(52,470,668),
            Point::new(216,146,977),
            Point::new(819,987,18),
            Point::new(117,168,530),
            Point::new(805,96,715),
            Point::new(346,949,466),
            Point::new(970,615,88),
            Point::new(941,993,340),
            Point::new(862,61,35),
            Point::new(984,92,344),
            Point::new(425,690,689)
        ];
        let act = read_input("inputs/day08/input1.txt");
    }

    #[test]
    fn test_solve1() {
        let edges = read_input("inputs/day08/input1.txt").unwrap();
        // println!("");
        // println!("");
        // println!("");
        let act = solve1(&edges, 10);
        let exp = 40;
        assert_eq!(exp, act);
    }
    #[test]
    fn test_solve2() {
        let edges = read_input("inputs/day08/input1.txt").unwrap();
        // println!("");
        // println!("");
        // println!("");
        let act = solve2(&edges, 1);
        let exp = 25272;
        assert_eq!(exp, act);
    }
}
