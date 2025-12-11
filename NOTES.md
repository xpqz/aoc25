## Matrix Exponentiation and Adjacency Matrices

For day 11:

For any adjacency matrix A:
- A[i,j] = 1 if edge from i to j
- (A²)[i,j] = number of paths of length exactly 2 from i to j
- (Aⁿ)[i,j] = number of paths of length exactly n from i to j

This follows from the definition of matrix multiplication: (A²)[i,j] = Σₖ A[i,k]·A[k,j], which sums over all intermediate nodes k.

## The Problem

We want paths of any length, not a fixed length. Naively you'd compute A + A² + A³ + ... but that's infinite.

## Halting Via Self-Loop

Adding a self-loop at destination d (setting M[d,d] = 1) makes d an absorbing state. Once a path reaches d, it "stays" there.

Now Mⁿ[i,d] counts walks of length n, but walks that reached d early just loop in place. Concretely:

| Path length | Contributes to M¹ | M² | M³ | M⁴ | ... |
|-------------|-------------------|----|----|----| --- |
| 1           | ✓                 | ✓  | ✓  | ✓  | ✓   |
| 2           |                   | ✓  | ✓  | ✓  | ✓   |
| 3           |                   |    | ✓  | ✓  | ✓   |

Once n ≥ (max path length), all paths are captured and Mⁿ[i,d] equals the total count. Further powers don't change it — fixed point reached.

## Why Squaring?

Repeated squaring (M → M² → M⁴ → M⁸...) is just a fast way to reach a high power. If the longest path has length L, you need only O(log L) iterations to exceed it. In a DAG with n nodes, L ≤ n−1, so convergence is fast.

## Worked Example

Graph: A → B → C and A → C (two paths from A to C)

```
g = [[0,1,1],    Add self-loop    m = [[0,1,1],
     [0,0,1],    at C (index 2)        [0,0,1],
     [0,0,0]]         →                [0,0,1]]
```

Compute m²[0,2] = m[0,0]·m[0,2] + m[0,1]·m[1,2] + m[0,2]·m[2,2]
                = 0 + 1·1 + 1·1 = 2

The two contributions: A→B→C (via k=1) and A→C→C (via k=2, using the self-loop).