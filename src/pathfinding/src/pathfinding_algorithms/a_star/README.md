# A\*

*A\** is a pathfinding algorithm that is very commonly used due to its great optimality and efficiency. It is essentially an extension of Dijkstra's shortest-path algorithm, but it uses heuristics to optimize finding the target and it only produces the shortest path to the target instead of a shortest-path tree.

*A\** maintains a tree of paths beginning from the source vertex. At each step the algorithm extends the path with the vertex that has the smallest total cost, which is calculated as the sum of the current cost of the path and the estimated cost required to extend the path to the target vertex. The estimated cost is computed using a given heuristic function, like the Euclidean distance function or the Manhattan distance function.

Typically a min-priority queue called *the open set* is used to efficiently get the next vertex with the smallest total cost. The algorithm stops when the target vertex is removed from the open set or there are no more paths left to extend. This produces just the length of the shortest path, but we can easily get the actual path by keeping track of each vertex's predecessor and at the end reconstructing the path from end to start.
