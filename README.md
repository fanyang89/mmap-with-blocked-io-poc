# POC for slow [mmap(2)][1]

## Environment setup

- 1c4g virtual machine
- Linux 3.10.0-957.el7.x86_64

## Expected behavior

Memory allocation delayed.

## Real world behavior

```
2024-08-02T16:44:33.884200455+08:00  INFO poc: mmap(2) elapsed: 552.019123ms
2024-08-02T16:44:34.906196913+08:00  INFO poc: mmap(2) elapsed: 1.001015008s
2024-08-02T16:44:35.467195248+08:00  INFO poc: mmap(2) elapsed: 550.014566ms
2024-08-02T16:44:36.026197497+08:00  INFO poc: mmap(2) elapsed: 548.01508ms
2024-08-02T16:44:37.118197043+08:00  INFO poc: mmap(2) elapsed: 540.016055ms
```

[1]: https://man7.org/linux/man-pages/man2/mmap.2.html
