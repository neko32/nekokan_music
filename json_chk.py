import json
from os import listdir
from pathlib import Path


def main() -> int:

    db_dir = Path("./db").absolute()

    failed_ones = []

    for file in listdir(db_dir):
        fpath = db_dir / file
        with open(fpath) as fp:
            try:
                json.load(fp)

            except Exception as e:
                failed_ones.append(str(fpath))
        
    if len(failed_ones) == 0:
        print("All good.")
        return 0
    else:
        print(f"Found {len(failed_ones)} errors.")
        for f in failed_ones:
            print(f" - {f}")
        return 1


if __name__ == "__main__":
    rez = main()
    if rez != 0:
        exit(1)
