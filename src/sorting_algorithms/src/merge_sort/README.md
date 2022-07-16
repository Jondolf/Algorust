# Merge sort

*Merge sort* is a commonly used efficient sorting algorithm based on the *divide-and-conquer* strategy. It can be implemented in multiple ways either iteratively or recursively. Most implementations of merge sort are stable.

First, merge sort recursively divides the input list in half until each sublist only contains one item. The sublists are then repeatedly merged in a sorted manner by comparing the items in the two halves and each time adding the minimum value into the merged list. Merging is continued until only one, sorted list remains.

## Performance

| Case             | Complexity |
| ---------------- | ---------- |
| Average          | O(n log n) |
| Worst-case       | O(n log n) |
| Best-case        | O(n log n) |
| Space complexity | O(n)       |
