#!/usr/bin/env python

import glob
import subprocess

subprocess.run(['tree-sitter', 'generate'])

for filename in glob.glob('./test/pmd-examples/valid/*'):
    print(f"Parsing {filename}")
    p = subprocess.run(['tree-sitter', 'parse', filename], stdout=subprocess.PIPE)
    if p.returncode != 0:
        print(f"Failed to parse {filename}")
        break

for filename in glob.glob('./test/pmd-examples/invalid/*'):
    print(f"Parsing {filename}")
    p = subprocess.run(['tree-sitter', 'parse', filename], stdout=subprocess.PIPE)
    if p.returncode == 0:
        print(f"Unexpectedly parsed {filename}")
        break

