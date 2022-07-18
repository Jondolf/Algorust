# Quicksort

*Quicksort*, sometimes also called *partition-exchange sort* is an in-place divide-and-conquer sorting algorithm. It is quite efficient, even performing better than merge sort and heapsort if implemented well. Most implementations of quicksort are not stable.

Quicksort selects a "pivot" element from the list, and partitions the list into two sublists according to whether the items are less than or greater than the chosen pivot element. This procedure is applied recursively to the two partitions until the list is sorted.

## Performance

| Case             | Complexity |
| ---------------- | ---------- |
| Average          | O(n log n) |
| Worst-case       | O(n²)      |
| Best-case        | O(n log n) |
| Space complexity | O(n)       |
