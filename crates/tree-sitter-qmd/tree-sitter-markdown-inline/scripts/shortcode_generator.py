#!/usr/bin/env python3

import sys
import random
import subprocess
import re

def generate_shortcode_name():
    return ("call", "(shortcode_name)") # TBF vary length

def generate_shortcode_naked_string():
    return ("val[]ue", "(shortcode_naked_string)") # TBF vary length and content

def generate_shortcode_boolean():
    return (random.choice(["true", "false"]), "(shortcode_boolean)")

def generate_shortcode_number():
    if random.random() < 0.5:
        return (str(random.randint(-100, 100)), "(shortcode_number)")
    else:
        return (str(random.uniform(-100.0, 100.0)), "(shortcode_number)")

def generate_shortcode_string():
    if random.random() < 0.5:
        return ('"' + generate_shortcode_naked_string()[0] + '"', "(shortcode_string)") # TBF add internal quotes
    else:
        return ("'" + generate_shortcode_naked_string()[0] + "'", "(shortcode_string)") # TBF add internal quotes

def generate_shortcode_key_value_pair():
    key_string, key_parse = generate_shortcode_name()
    value_string, value_parse = generate_shortcode_positional_argument()
    spacer1 = "".join([" "] * random.randint(0, 2))
    spacer2 = "".join([" "] * random.randint(0, 2))
    return (f"{key_string}{spacer1}={spacer2}{value_string}", f"(shortcode_keyword_param{key_parse}{value_parse})")

def generate_shortcode_positional_argument():
    return random.choice([
        generate_shortcode_naked_string,
        generate_shortcode_boolean,
        generate_shortcode_number,
        generate_shortcode_string,
        generate_shortcode
    ])()

def generate_shortcode(escaped=False):
    if escaped:
        strs = [r"{{{< "]
        node_name = "shortcode_escaped"
    else:
        strs = [r"{{< "]
        node_name = "shortcode"
    parses = []
    name_string, name_parse = generate_shortcode_name()
    strs.append(name_string)
    parses.append(name_parse)
    while random.random() < 0.5:
        strs.append(" ")
        arg_string, arg_parse = generate_shortcode_positional_argument()
        strs.append(arg_string)
        parses.append(arg_parse)
    while random.random() < 0.5:
        strs.append(" ")
        arg_string, arg_parse = generate_shortcode_key_value_pair()
        strs.append(arg_string)
        parses.append(arg_parse)
    if escaped:
        strs.append(r" >}}}")
    else:
        strs.append(r" >}}")
    
    return ("".join(strs), f"({node_name}(shortcode_delimiter)" + "".join(parses) + "(shortcode_delimiter))")

if __name__ == "__main__":
    # For debugging, run tree-sitter parse
    while True:
        escaped = random.random() < 0.1
        shortcode, shortcode_parse = generate_shortcode(escaped)
        # add scaffolding to shortcode_parse
        shortcode_parse = "(inline" + shortcode_parse + ")"
        try:
            output = subprocess.run(["tree-sitter", "parse", "--no-ranges"], input=shortcode.encode(), capture_output=True, check=True)
            output_str = output.stdout.decode('utf-8').strip()
            # remove all whitespace from output_str
            output_str = re.sub(r'\s+', '', output_str)
            if output_str != shortcode_parse:
                print(f"\nMismatch:\n{shortcode}\nparse: {output_str}\nexpectation: {shortcode_parse}", file=sys.stderr)
                sys.exit(1)
            sys.stderr.write(".")
            sys.stderr.flush()
        except subprocess.CalledProcessError as e:
            print(f"Error running tree-sitter: {e}", file=sys.stderr)
            print(shortcode, file=sys.stderr)
            print("\n", file=sys.stderr)

            sys.exit(1)