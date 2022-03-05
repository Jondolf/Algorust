# Heapsort

*Heapsort* is a comparison-based in-place sorting algorithm based on the binary heap data structure. After forming the heap, it divides its input into a sorted and an unsorted section, iterating over the unsorted section and moving its largest element to the start of the sorted section.

Heapsort is very similar to selection sort, but it uses a heap data structure for finding the largest element instead of a linear-time search.

Most implementations of heapsort are unstable sorts.

## Performance

| Case             | Complexity                 |
| ---------------- | -------------------------- |
| Average          | O(n log n)                 |
| Worst-case       | O(n log n)                 |
| Best-case        | O(n log n)                 |
| Space complexity | O(n) total, O(1) auxiliary |
