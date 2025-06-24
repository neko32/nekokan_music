from os import getenv, listdir, unlink
from pathlib import Path
from shutil import copyfile

module_name = "musicdb"


def get_resource_dir() -> str:

    maybe_rc_dir = getenv("NEKORC_PATH")
    if maybe_rc_dir is None:
        raise RuntimeError("NEKORC_PATH must be set")
    return maybe_rc_dir



def main():
    global module_name
    install_dest_base_path = str(Path(get_resource_dir()) / module_name)
    print(f"publishing resource for the project {install_dest_base_path} [module:{module_name}] ... ")

    try:
        for f in listdir(install_dest_base_path):
            fpath = str(Path(install_dest_base_path) / f)
            print(f"unlinking {fpath} .. ")
            unlink(fpath)
    except Exception as e:
        print(f"unlinking failed - {str(e)}")
    
    print("\n\nall unlinking existing files done. Installing new ones..")

    db_path = Path("db")
    try:
        for f in listdir(db_path):
            fpath = str(Path(db_path) / f)
            dest_path = str(Path(install_dest_base_path) / f)
            print(f"installing {fpath} to {dest_path} ... ")
            copyfile(fpath, dest_path)
    except Exception as e:
        print(f"installation failed - {str(e)}")
        

    print("\n\nnew file installation done.")


    print(f"publishing resource for the project {install_dest_base_path} done successfully.")


if __name__ == "__main__":
    main()
