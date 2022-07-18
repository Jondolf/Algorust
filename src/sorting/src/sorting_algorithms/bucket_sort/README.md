# Bucket sort

*Bucket sort* or *bin sort* is a sorting algorithm that distributes the items in a list into a number of "buckets" that are sorted individually with another sorting algorithm and combined at the end.

Bucket sort's efficiency depends on the number and size of the buckets, the other algorithm that is used, and how uniformly the input is distributed.

## Performance

Here *k* denotes the number of buckets.

| Case             | Complexity      |
| ---------------- | --------------- |
| Average          | O(n + n²/k + k) |
| Worst-case       | O(n²)           |
| Space complexity | O(n + k)        |
