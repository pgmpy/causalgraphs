use extendr_api::prelude::*;
use rust_core::RustDAG;
use extendr_api::Nullable;

pub struct RDAG {
    inner: RustDAG,
}

#[extendr]
impl RDAG {
    fn new() -> Self {
        RDAG { inner: RustDAG::new() }
    }

    fn add_node(&mut self, node: String, latent: Option<bool>) -> extendr_api::Result<()> {
        self.inner.add_node(node, latent.unwrap_or(false))
            .map_err(Error::from)
    }

    fn add_nodes_from(&mut self, nodes: Vec<String>, latent: Nullable<Vec<i32>>) -> extendr_api::Result<()> {
        let latent_opt: Option<Vec<bool>> = latent.into_option().map(|v| v.into_iter().map(|x| x != 0).collect());
        self.inner.add_nodes_from(nodes, latent_opt)
            .map_err(Error::from)
    }

    fn add_edge(&mut self, u: String, v: String, weight: Option<f64>) -> extendr_api::Result<()> {
        self.inner.add_edge(u, v, weight)
            .map_err(Error::from)
    }

    fn get_parents(&self, node: String) -> extendr_api::Result<Vec<String>> {
        self.inner.get_parents(&node)
            .map_err(Error::from)
    }

    fn get_children(&self, node: String) -> extendr_api::Result<Vec<String>> {
        self.inner.get_children(&node)
            .map_err(Error::from)
    }

    fn get_ancestors_of(&self, nodes: Vec<String>) -> extendr_api::Result<Vec<String>> {
        Ok(self.inner.get_ancestors_of(nodes)
            .map_err(Error::from)?
            .into_iter().collect())
    }

    fn nodes(&self) -> Vec<String> {
        self.inner.nodes()
    }

    fn edges(&self) -> List {
        let edges = self.inner.edges();
        let (from, to): (Vec<_>, Vec<_>) = edges.into_iter().unzip();
        list!(from = from, to = to)
    }

    fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    fn latents(&self) -> Vec<String> {
        self.inner.latents.iter().cloned().collect()
    }
}

extendr_module! {
    mod causalgraphs;
    impl RDAG;
}