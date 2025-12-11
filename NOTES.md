## Reachability and Transitive Closure

**Transitive closure** (can i reach j at all?): Same idea as path counting, but with boolean operations.

```python
M = (G + np.eye(n)) > 0  # add self-loops
while not converged:
    M = (M @ M) > 0
```

This is O(n³ log n) with dense matrices. Warshall's algorithm (the k-loop variant) is O(n³) with better constants. For sparse graphs, BFS from each node beats both.

## Shortest Paths

Two related approaches:

**Floyd-Warshall** (the standard k-loop algorithm):
```python
D = G.copy()  # edge weights, inf if no edge
for k in range(n):
    D = np.minimum(D, np.add.outer(D[:,k], D[k,:]))
```

**Min-plus repeated squaring** (different algorithm, same result):
```python
def min_plus(A, B):
    return np.min(A[:,:,None] + B[None,:,:], axis=1)

D = G.copy()
while not converged:
    D = min_plus(D, D)
```

Both use the (min, +) semiring. Floyd-Warshall is O(n³); repeated squaring is O(n³ log n) but parallelizes better.

## Connected Components (Undirected)

Compute the transitive closure (boolean squaring until fixed point). In the resulting matrix, identical rows indicate same component membership.

The spectral method is more common in practice: cluster on the Laplacian's eigenvectors (see below).

## PageRank / Eigenvector Centrality

Dominant eigenvector of a modified adjacency matrix:

```python
# Handle dangling nodes (no outlinks)
out_degree = G.sum(axis=0)
out_degree[out_degree == 0] = 1  # avoid division by zero
M = G / out_degree  # column stochastic

# Power iteration with damping factor
alpha = 0.85
v = np.ones(n) / n
for _ in range(100):
    v = alpha * (M @ v) + (1 - alpha) / n
```

The damping factor ensures convergence and models random jumps. Without it, dangling nodes and disconnected components cause problems.

## Graph Laplacian

L = D − A (degree matrix minus adjacency). Eigenvalues reveal structure:

| Property | What it tells you |
|----------|-------------------|
| # of zero eigenvalues | # of connected components |
| Second-smallest eigenvalue (Fiedler) | Connectivity strength |
| Eigenvectors | Spectral clustering coordinates |

```python
D = np.diag(G.sum(axis=1))
L = D - G
eigvals, eigvecs = np.linalg.eigh(L)
```

Numerical note: computing eigenvalues of large matrices can be unstable. Use sparse solvers for the few smallest eigenvalues you actually need.

## Counting Triangles

```python
triangles = np.trace(G @ G @ G) // 6
```

(G³)[i,i] counts closed walks of length 3 from i. Divide by 6 because each triangle is counted 3 (starting nodes) × 2 (directions).

## Counting Spanning Trees

**Kirchhoff's theorem**: Delete any row/column from the Laplacian, take the determinant.

```python
num_spanning_trees = round(np.linalg.det(L[1:, 1:]))
```

Numerical note: determinants are unstable for large matrices. Use LU decomposition and sum log-diagonals for better precision.

## Bipartiteness Check

Graph is bipartite iff all eigenvalues of A are symmetric around 0 (λ is an eigenvalue ⟺ −λ is). This works for disconnected graphs too — the spectrum is the union of component spectra.

In practice, BFS with 2-coloring is simpler and faster.

## Maximum Matching (Bipartite)

For bipartite graph with biadjacency matrix B, the rank of B does **not** give the matching size directly. Counterexample:

```
K_{2,2}: B = [[1,1],    rank = 1
              [1,1]]    max matching = 2
```

The correct formulation uses the **Tutte matrix**: replace each 1 with a distinct symbolic variable xᵢⱼ. Then rank over the field of rational functions equals max matching size. In practice, substitute random integers and compute rank — correct with high probability.

For actual implementation, use Hopcroft-Karp (O(E√V)) instead.

## Summary Table

| Problem | Matrix Operation | Practical? |
|---------|------------------|------------|
| Path counting | Aⁿ with absorbing state | Small graphs |
| Reachability | Boolean closure (A+I)* | BFS better |
| Shortest paths | (min,+) multiplication | Floyd-Warshall or Dijkstra |
| Components | Transitive closure rows | BFS/DFS better |
| Centrality | Dominant eigenvector | Yes, with damping |
| Clustering | Laplacian eigenvectors | Yes |
| Triangles | tr(A³)/6 | Yes |
| Spanning trees | det(Laplacian minor) | Moderate n |
| Bipartiteness | Spectrum symmetry | BFS 2-coloring better |
| Max matching | Tutte matrix rank | Hopcroft-Karp better |

The matrix perspective unifies these problems algebraically and maps well to GPU parallelism. But for sparse graphs, specialized algorithms usually win.