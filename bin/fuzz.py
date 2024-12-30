from itertools import product
from multiprocessing import Pool
import subprocess

# from ' ' to '~'
CHARS = [chr(i) for i in range(32, 127)]


def run_test(data: tuple[str]):
    n = int(data[0])
    print(f'Running tests for strings of length {n}')
    # enumerate strings of length n
    for i in range(1, n + 1):
        for s in product(CHARS, repeat=i):
            inp = "".join(s)
            try:
                subprocess.run(
                    ["./target/release/oxynium", "-o=test-out", f'-e="{inp}"'],
                    check=True,
                    capture_output=True
                )
            except subprocess.CalledProcessError as e:
                print(f"Test failed for {inp}")


def main():
    with Pool() as pool:
        pool.map(run_test, list(map(str, range(1, 6))))


if __name__ == "__main__":
    main()
