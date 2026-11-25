//! Pure graph/tree algorithms for the "Pohon (Tree)" learning page.
//!
//! These functions are intentionally free of any UI or I/O so they can be
//! unit-tested in isolation. The interactive page re-implements the same
//! algorithms in JavaScript (required because the generated site is static
//! HTML with no backend), and this module serves as the tested reference.

/// A weighted undirected edge. `u` and `v` are vertex indices in `0..n`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub w: i64,
}

impl Edge {
    pub fn new(u: usize, v: usize, w: i64) -> Self {
        Edge { u, v, w }
    }
}

/// Disjoint-set (union-find) with path compression and union by rank.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // Path compression.
        let mut cur = x;
        while self.parent[cur] != cur {
            let next = self.parent[cur];
            self.parent[cur] = root;
            cur = next;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        true
    }
}

/// Connected components as lists of vertex indices.
pub fn components(n: usize, edges: &[Edge]) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    for e in edges {
        adj[e.u].push(e.v);
        adj[e.v].push(e.u);
    }
    let mut seen = vec![false; n];
    let mut comps = Vec::new();
    for s in 0..n {
        if seen[s] {
            continue;
        }
        let mut stack = vec![s];
        seen[s] = true;
        let mut comp = Vec::new();
        while let Some(x) = stack.pop() {
            comp.push(x);
            for &y in &adj[x] {
                if !seen[y] {
                    seen[y] = true;
                    stack.push(y);
                }
            }
        }
        comp.sort_unstable();
        comps.push(comp);
    }
    comps.sort();
    comps
}

/// True if the undirected (multi)graph contains at least one cycle.
/// Self-loops and parallel edges are treated as cycles.
pub fn has_cycle(n: usize, edges: &[Edge]) -> bool {
    let mut uf = UnionFind::new(n);
    for e in edges {
        if e.u == e.v {
            return true; // self-loop
        }
        if !uf.union(e.u, e.v) {
            return true; // endpoints already connected -> cycle
        }
    }
    false
}

/// Indices of edges that are bridges (removing them disconnects the graph).
/// Uses Tarjan's low-link algorithm; safe for multigraphs via edge ids.
pub fn bridges(n: usize, edges: &[Edge]) -> Vec<usize> {
    let m = edges.len();
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (i, e) in edges.iter().enumerate() {
        adj[e.u].push((e.v, i));
        adj[e.v].push((e.u, i));
    }
    let mut disc = vec![0usize; n];
    let mut low = vec![0usize; n];
    let mut visited = vec![false; n];
    let mut timer = 0usize;
    let mut is_bridge = vec![false; m];

    // Iterative DFS to avoid recursion-depth concerns.
    // Stack holds (vertex, parent_edge, iterator_index).
    for start in 0..n {
        if visited[start] {
            continue;
        }
        visited[start] = true;
        timer += 1;
        disc[start] = timer;
        low[start] = timer;
        let mut stack: Vec<(usize, usize, usize)> = vec![(start, usize::MAX, 0)];
        while let Some(&mut (v, _pe, ref mut idx)) = stack.last_mut() {
            if *idx < adj[v].len() {
                let (to, eid) = adj[v][*idx];
                *idx += 1;
                if eid == _pe {
                    continue;
                }
                if !visited[to] {
                    visited[to] = true;
                    timer += 1;
                    disc[to] = timer;
                    low[to] = timer;
                    stack.push((to, eid, 0));
                } else {
                    low[v] = low[v].min(disc[to]);
                }
            } else {
                let (v, pe, _) = stack.pop().unwrap();
                if let Some(&(parent, _, _)) = stack.last() {
                    low[parent] = low[parent].min(low[v]);
                    if low[v] > disc[parent] {
                        is_bridge[pe] = true;
                    }
                }
            }
        }
    }
    (0..m).filter(|&i| is_bridge[i]).collect()
}

/// A graph is a tree iff it is connected, acyclic and has exactly n-1 edges.
pub fn is_tree(n: usize, edges: &[Edge]) -> bool {
    if n == 0 {
        return false;
    }
    let connected = components(n, edges).len() == 1;
    connected && edges.len() + 1 == n && !has_cycle(n, edges)
}

/// Result of a minimum-spanning-tree style computation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MstResult {
    pub edges: Vec<usize>,
    pub total: i64,
    pub connected: bool,
}

/// Prim's algorithm. `start` is the source vertex. For a disconnected graph
/// only the component reachable from `start` is spanned (`connected=false`).
pub fn prim(n: usize, edges: &[Edge], start: usize) -> MstResult {
    if n == 0 {
        return MstResult { edges: vec![], total: 0, connected: true };
    }
    let mut adj: Vec<Vec<(usize, i64, usize)>> = vec![Vec::new(); n];
    for (i, e) in edges.iter().enumerate() {
        adj[e.u].push((e.v, e.w, i));
        adj[e.v].push((e.u, e.w, i));
    }
    let mut in_tree = vec![false; n];
    let mut used_edge = vec![false; edges.len()];
    let mut chosen = Vec::new();
    let mut total = 0i64;
    in_tree[start] = true;
    let mut visited_count = 1;

    while visited_count < n {
        // Pick the minimum-weight edge crossing the cut (u in tree, v not).
        let mut best: Option<(i64, usize, usize)> = None; // (w, edge_id, to)
        for u in 0..n {
            if !in_tree[u] {
                continue;
            }
            for &(v, w, eid) in &adj[u] {
                if !in_tree[v] && !used_edge[eid] {
                    match best {
                        Some((bw, _, _)) if bw <= w => {}
                        _ => best = Some((w, eid, v)),
                    }
                }
            }
        }
        match best {
            Some((w, eid, v)) => {
                used_edge[eid] = true;
                in_tree[v] = true;
                visited_count += 1;
                chosen.push(eid);
                total += w;
            }
            None => break, // disconnected
        }
    }
    MstResult { edges: chosen, total, connected: visited_count == n }
}

/// Kruskal's algorithm with union-find. Edges are processed by ascending
/// weight (ties broken by original order for determinism).
pub fn kruskal(n: usize, edges: &[Edge]) -> MstResult {
    let mut order: Vec<usize> = (0..edges.len()).collect();
    order.sort_by_key(|&i| (edges[i].w, i));
    let mut uf = UnionFind::new(n);
    let mut chosen = Vec::new();
    let mut total = 0i64;
    for i in order {
        let e = edges[i];
        if uf.union(e.u, e.v) {
            chosen.push(i);
            total += e.w;
            if chosen.len() + 1 == n {
                break;
            }
        }
    }
    let connected = chosen.len() + 1 == n || n == 1;
    MstResult { edges: chosen, total, connected }
}

/// A rooted-tree view produced from an undirected tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootedTree {
    pub root: usize,
    pub parent: Vec<Option<usize>>,
    pub children: Vec<Vec<usize>>,
    pub depth: Vec<usize>,
    pub indeg: Vec<usize>,
    pub outdeg: Vec<usize>,
    pub height: usize,
    pub order: Vec<usize>, // BFS order from root
}

/// Root the tree at `root`. Assumes the input is a valid tree (caller should
/// validate with `is_tree`); on invalid input the result is best-effort.
pub fn rooted(n: usize, edges: &[Edge], root: usize) -> RootedTree {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in edges {
        adj[e.u].push(e.v);
        adj[e.v].push(e.u);
    }
    let mut parent = vec![None; n];
    let mut depth = vec![0usize; n];
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut order = Vec::new();
    let mut seen = vec![false; n];
    if n > 0 {
        seen[root] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while let Some(v) = queue.pop_front() {
            order.push(v);
            for &to in &adj[v] {
                if !seen[to] {
                    seen[to] = true;
                    parent[to] = Some(v);
                    depth[to] = depth[v] + 1;
                    children[v].push(to);
                    queue.push_back(to);
                }
            }
        }
    }
    let height = depth.iter().copied().max().unwrap_or(0);
    let mut indeg = vec![0usize; n];
    let mut outdeg = vec![0usize; n];
    for v in 0..n {
        if parent[v].is_some() {
            indeg[v] = 1;
        }
        outdeg[v] = children[v].len();
    }
    RootedTree { root, parent, children, depth, indeg, outdeg, height, order }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(u: usize, v: usize, w: i64) -> Edge {
        Edge::new(u, v, w)
    }

    // Contoh 3.1: 8 vertices a..h (0..7)
    fn contoh_31() -> (usize, Vec<Edge>) {
        let edges = vec![
            e(0, 2, 4), // a-c
            e(0, 1, 5), // a-b
            e(2, 1, 3), // c-b
            e(0, 3, 4), // a-d
            e(0, 4, 4), // a-e
            e(1, 4, 5), // b-e
            e(3, 4, 5), // d-e
            e(3, 7, 5), // d-h
            e(4, 6, 4), // e-g
            e(7, 6, 4), // h-g
            e(4, 5, 3), // e-f
            e(5, 6, 2), // f-g
        ];
        (8, edges)
    }

    #[test]
    fn contoh_31_kruskal_weight_24() {
        let (n, edges) = contoh_31();
        let r = kruskal(n, &edges);
        assert_eq!(r.total, 24, "Contoh 3.1 MST weight must be 24");
        assert!(r.connected);
        assert_eq!(r.edges.len(), n - 1);
    }

    #[test]
    fn contoh_31_prim_weight_24() {
        let (n, edges) = contoh_31();
        let r = prim(n, &edges, 0);
        assert_eq!(r.total, 24);
        assert!(r.connected);
        assert_eq!(r.edges.len(), n - 1);
    }

    #[test]
    fn prim_and_kruskal_equal_weight() {
        let (n, edges) = contoh_31();
        assert_eq!(prim(n, &edges, 0).total, kruskal(n, &edges).total);
        // also from a different start vertex
        assert_eq!(prim(n, &edges, 5).total, kruskal(n, &edges).total);
    }

    #[test]
    fn cycle_graph_is_not_tree() {
        // triangle a-b-c-a plus a pendant d
        let edges = vec![e(0, 1, 1), e(1, 2, 1), e(2, 0, 1), e(0, 3, 1)];
        assert!(has_cycle(4, &edges));
        assert!(!is_tree(4, &edges));
    }

    #[test]
    fn forest_two_components_detected() {
        // component 1: a-b, component 2: c-d-e
        let edges = vec![e(0, 1, 1), e(2, 3, 1), e(3, 4, 1)];
        let comps = components(5, &edges);
        assert_eq!(comps.len(), 2);
        assert_eq!(comps[0], vec![0, 1]);
        assert_eq!(comps[1], vec![2, 3, 4]);
        assert!(!has_cycle(5, &edges));
        assert!(!is_tree(5, &edges)); // disconnected -> not a tree
    }

    #[test]
    fn simple_path_is_tree_and_bridges() {
        let edges = vec![e(0, 1, 1), e(1, 2, 1), e(2, 3, 1)];
        assert!(is_tree(4, &edges));
        // every edge of a tree is a bridge
        assert_eq!(bridges(4, &edges), vec![0, 1, 2]);
    }

    #[test]
    fn cycle_edge_not_bridge() {
        // triangle: no bridges
        let edges = vec![e(0, 1, 1), e(1, 2, 1), e(2, 0, 1)];
        assert!(bridges(3, &edges).is_empty());
    }

    #[test]
    fn rooted_tree_degrees() {
        // star: 0 center, 1..3 leaves, rooted at 0
        let edges = vec![e(0, 1, 1), e(0, 2, 1), e(0, 3, 1)];
        let t = rooted(4, &edges, 0);
        assert_eq!(t.root, 0);
        assert_eq!(t.indeg[0], 0);
        assert_eq!(t.outdeg[0], 3);
        assert_eq!(t.height, 1);
        for v in 1..4 {
            assert_eq!(t.indeg[v], 1);
            assert_eq!(t.outdeg[v], 0); // leaves
        }
        // re-root at a leaf: height becomes 2
        let t2 = rooted(4, &edges, 1);
        assert_eq!(t2.height, 2);
        assert_eq!(t2.parent[0], Some(1));
    }

    #[test]
    fn disconnected_prim_reports_not_connected() {
        let edges = vec![e(0, 1, 1), e(2, 3, 1)];
        let r = prim(4, &edges, 0);
        assert!(!r.connected);
        assert_eq!(r.edges.len(), 1);
        let k = kruskal(4, &edges);
        assert!(!k.connected);
    }
}