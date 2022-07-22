# Depth-first search (DFS)

*Depth-first search* or *DFS* is an algorithm used for searching or traversing tree and graph data structures. It starts at the given source vertex, and explores each branch as far as possible before backtracking. Typical applications of DFS include topological sorting, finding connected components, and solving and generating mazes.

DFS can be implemented either recursively or iteratively, and these two variations visit the neighbors of each vertex in the opposite order from each other.

In the recursive approach, the visited vertices are supplied by an argument. The current vertex is marked as visited, and the function is called recursively for all adjacent vertices that have not been visited yet.

The iterative approach on the other hand uses a stack, initialized with just the source vertex. At each iteration, a vertex is popped from the stack, and if it has not been visited, its adjacent vertices are pushed to the stack. This method is similar to *breadth-first search* or *BFS*, but DFS uses a stack instead of a queue, and it checks if the vertex is visited after it has been popped from the stack rather than before it has been pushed to the stack.
