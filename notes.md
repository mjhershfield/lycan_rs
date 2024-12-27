Old bitstream works correctly
```
Wrote 16 bytes: [1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F, 10]
Wrote 16 bytes: [D, E, F, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1A, 1B, 1C]
Read 16 bytes: [1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F, 10]
Read 16 bytes: [D, E, F, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1A, 1B, 1C]
```

For new bitstream, values are corrupted but eventually are correct. Timing problem?
```
Wrote 16 bytes: [1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F, 10]
Wrote 16 bytes: [D, E, F, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1A, 1B, 1C]
Read 16 bytes: [1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, 63, E5, 51, FC]
Read 16 bytes: [D, E, F, 10, C6, D0, F4, B9, 11, 12, 13, 14, 15, 16, 17, 18]
```
```
Wrote 16 bytes: [1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F, 10]
Wrote 16 bytes: [D, E, F, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1A, 1B, 1C]
Read 16 bytes: [1, 2, 3, 4, 63, E5, 51, FC, C6, D0, F4, B9, 5, 6, 7, 8]
Read 16 bytes: [D, E, F, 10, D, E, F, 10, 11, 12, 13, 14, 15, 16, 17, 18]
```