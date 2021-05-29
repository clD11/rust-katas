#[derive(Debug)]
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn build(v: usize) -> Graph {
        let graph = Graph {
            adj: vec![vec![]; v],
        };
        graph
    }

    fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.adj[w].push(v);
    }

    fn adj(&mut self, v: usize) -> &Vec<usize> {
        return &self.adj[v];
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_initialize_graph() {
        let vertices = 5;
        let actual = Graph::build(vertices);
        assert_eq!(actual.adj.len(), vertices);
    }

    #[test]
    fn should_add_edge() {
        let vertices = 5;
        let mut actual = Graph::build(vertices);
        actual.add_edge(0, 1);
        assert_eq!(actual.adj[0][0], 1);
        assert_eq!(actual.adj[1][0], 0);
    }

    #[test]
    fn should_get_adj_for_vertex() {
        let vertices = 5;
        let mut graph = Graph::build(vertices);
        let vertex = 0;
        graph.add_edge(vertex, 1);
        let actual = graph.adj(vertex);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], 1);
    }
}
