from multiprocessing import Pool
import subprocess


def run_test(data: tuple[str]) -> tuple[bool, str, str]:
    name = data[0]
    print(f"Running '{name}'...")
    cmd = ["sudo", "docker", "run", f"oxy-spec-{name}"]
    try:
        return (
            True,
            subprocess.run(cmd, check=True, capture_output=True).stdout.decode(),
            name
        )
    except subprocess.CalledProcessError as e:
        return False, str(e.output), name


def main():
    fails = False
    with Pool() as pool:
        result = pool.map(run_test, [
            ("ubuntu",),
            ("arch",),
            ("alpine",),
            ("debian",),
            ("macos",),
        ])
        for success, output, name in result:
            out = (u'' + output)\
                .encode('latin-1', 'backslashreplace')\
                .decode('unicode-escape')
            with open(f"./test/spec-{name}.log", "a") as f:
                f.write(out)
            if not success:
                fails = True
                print("Test Failed!")
                print(out)
                print("----------------------")

    if fails:
        raise Exception("At least one test failed")
    else:
        print("All tests passed")


if __name__ == "__main__":
    main()
