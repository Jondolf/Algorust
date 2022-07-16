# Insertion sort

*Insertion sort* is a very simple in-place sorting algorithm that builds the final sorted list by iteratively growing the sorted part of the list. At each iteration, the algorithm takes an item from the unsorted part, finds its appropriate location in the sorted part, and inserts the item there. This is repeated until the entire list has been iterated through.

Insertion sort is inefficient for sorting large lists, and algorithms such as merge sort, quicksort or heapsort should be used instead. It's primary advantages are that it is very easy to implement, it is stable, and it is efficient for small or mostly sorted data sets.

## Performance

| Case             | Complexity |
| ---------------- | ---------- |
| Average          | O(n²)      |
| Worst-case       | O(n²)      |
| Best-case        | O(n)       |
