"""
Python wrapper for Rust DAG implementation with pgmpy compatibility
"""
from typing import List, Set, Optional, Union, Hashable
from .pgmpy_rust import RustDAG as _RustDAG


class RustDAG:
    """
    A high-performance DAG implementation using Rust backend.
    Compatible with pgmpy's DAG interface.
    """
    
    def __init__(self, ebunch=None, latents=None):
        self._dag = _RustDAG()
        self.latents = set(latents or [])
        
        if ebunch:
            for u, v in ebunch:
                self.add_edge(str(u), str(v))
    
    def add_node(self, node: Hashable, latent: bool = False):
        """Add a single node to the graph."""
        self._dag.add_node(str(node), latent)
        if latent:
            self.latents.add(str(node))
    
    def add_nodes_from(self, nodes: List[Hashable], latent: Union[bool, List[bool]] = False):
        """Add multiple nodes to the graph."""
        nodes_str = [str(n) for n in nodes]
        
        if isinstance(latent, bool):
            latent_flags = [latent] * len(nodes)
        else:
            latent_flags = latent
            
        self._dag.add_nodes_from(nodes_str, latent_flags)
        
        for node, is_latent in zip(nodes_str, latent_flags):
            if is_latent:
                self.latents.add(node)
    
    def add_edge(self, u: Hashable, v: Hashable, weight: Optional[float] = None):
        """Add an edge between two nodes."""
        self._dag.add_edge(str(u), str(v), weight)
    
    def get_parents(self, node: Hashable) -> List[str]:
        """Get parents of a node."""
        return self._dag.get_parents(str(node))
    
    def get_children(self, node: Hashable) -> List[str]:  
        """Get children of a node."""
        return self._dag.get_children(str(node))
    
    def _get_ancestors_of(self, nodes: Union[str, List[Hashable]]) -> Set[str]:
        """Get ancestors of given nodes (optimized Rust implementation)."""
        if isinstance(nodes, str):
            nodes = [nodes]
        nodes_str = [str(n) for n in nodes]
        return self._dag.get_ancestors_of(nodes_str)
    
    # def minimal_dseparator(self, start: Hashable, end: Hashable, 
    #                       include_latents: bool = False) -> Optional[Set[str]]:
    #     """Find minimal d-separating set (optimized Rust implementation)."""
    #     return self._dag.minimal_dseparator(str(start), str(end), include_latents)
    
    # def is_dconnected(self, start: Hashable, end: Hashable, 
    #                  observed: Optional[List[Hashable]] = None) -> bool:
    #     """Check if two nodes are d-connected."""
    #     observed_str = [str(n) for n in observed] if observed else None
    #     return self._dag.is_dconnected(str(start), str(end), observed_str)
    
    def nodes(self) -> List[str]:
        """Get all nodes."""
        return self._dag.nodes()
    
    def edges(self) -> List[tuple]:
        """Get all edges."""
        return self._dag.edges()
    
    def __len__(self) -> int:
        """Return number of nodes."""
        return self._dag.node_count()
    
    def number_of_edges(self) -> int:
        """Return number of edges."""
        return self._dag.edge_count()