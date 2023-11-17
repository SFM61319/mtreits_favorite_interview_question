## My solution to [mtreit's favorite interview question](https://treit.github.io/programming,/interviewing/2023/02/03/InterviewQuestion.html)

To run the program, run the following command from the project root directory:

```sh
$ cargo run --release -- <filename1>.bin <filename2>.bin <filename_out>.bin
```

You can also run the following script to generate random binary files:

```py
import random

def generate_random_binary_file(filename: str, count: int):
    with open(filename, 'wb') as f:
        for _ in range(count):
            random_bytes = random.randbytes(4)
            f.write(random_bytes)

def main(argv: list[str]) -> int:
    if len(argv) < 3:
        print("Usage: python rng.py <filename>.bin <count>")
        return 1

    generate_random_binary_file(argv[1], int(argv[2]))
    return 0


if __name__ == '__main__':
    import sys

    code = main(sys.argv)
    sys.exit(code)
```

And run it as:

```sh
$ python rng.py <filename>.bin <count>
```
