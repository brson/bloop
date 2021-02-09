#!/bin/python3

import suprocess;

res = subprocess.run(["cargo", "build", "--release"])
if res.returncode != 0:
    print(res.stdout)
    print(res.stderr)
    sys.exit(-1)
