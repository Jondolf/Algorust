# Dijkstra

*Dijkstra's shortest path algorithm* is a commonly used pathfinding algorithm created by Edsger W. Dijkstra in 1956. It finds the shortest paths from a given source vertex to all other vertices in the given graph, producing a *shortest-path tree*. It works for all graphs with non-negative edge weights.

Dijkstra's algorithm typically uses a min-priority queue to store the nearest unvisited vertices. The queue is initialized with only the source vertex. At each step we take the next vertex with the smallest distance from the source vertex and mark it as visited. Next we iterate through its neighbors, comparing the distance values along the current path to their previous minimum distance values. If the new distance is shorter, the neighbor's distance value is updated and the vertex is added to the queue. This process is repeated until the queue is empty. If we are only interested in the shortest path to the target vertex, we can also stop when it is found.
