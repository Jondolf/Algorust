# Rust algorithms

This is a website with interactive visualizations of various sorting algorithms. The entire project is made in Rust, with a [Yew](https://yew.rs) frontend. This was created mainly for learning purposes, but I may eventually publish it as a proper website.

## Implemented algorithms

Below are all currently implemented algorithms.

### Sorting

- Bubble sort
- Bucket sort
- Heapsort
- Insertion sort
- Merge sort
- Quicksort

Below is an image of a sorting algorithm's page with a bar graph of a randomly generated input. You can go through the steps that the algorithm takes to sort the input by using the slider.

![A sorting algorithm's page with a bar graph of random numbers.](/assets/images/sorting.png)

### Pathfinding

- Depth-first search / DFS (unweighted, doesn't guarantee shortest path)
- Dijkstra (weighted, guarantees shortest path)
- A* (weighted, uses heuristic, generally guarantees shortest path)
- More coming soon

Below is a screenshot of running the dijkstra pathfinding algorithm in a drawn labyrinth. It shows the visited positions at each step, and when you get to the final step, you will see the finished path.
![A pathfinding algorithm's page with an algorithm looking for a path within a labyrinth.](/assets/images/pathfinding.png)
