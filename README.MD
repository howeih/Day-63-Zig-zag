Day 63: Zig-zag
<br>
Zig-zag scan of a matrix is useful when you transform your 2D data between domains and the transformation tends to accumulate information into top left corner.
<br>
An example is JPEG image compression with discrete cosine transform, which accumulates energy to the left [in 1D] and top [in 2D].
<br>

![Alt text](zigzag.png?raw=true "zigzag")

run:
```

fn main() {
    let zig_zag = ZigZag::new(10, 10);
    zig_zag.zig_zag();
    zig_zag.print_zig_zag();
}

```

result:
```

    0    1    5    6   14   15   27   28   44   45
    2    4    7   13   16   26   29   43   46   63
    3    8   12   17   25   30   42   47   62   64
    9   11   18   24   31   41   48   61   65   78
   10   19   23   32   40   49   60   66   77   79
   20   22   33   39   50   59   67   76   80   89
   21   34   38   51   58   68   75   81   88   90
   35   37   52   57   69   74   82   87   91   96
   36   53   56   70   73   83   86   92   95   97
   54   55   71   72   84   85   93   94   98   99


```