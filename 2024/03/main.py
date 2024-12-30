import os
import re
import sys

from itertools import chain
    
if __name__ == '__main__':
    data = list()
    infile = sys.argv[1]

    # read file    
    with open(infile, 'r') as f:
        for line in f.readlines():
            data.append(line)

    # match pattern
    pattern = r"mul\((\d+),(\d+)\)(.*?)(?=mul\(|$)"
    subpattern = r"(do\(\)|don't\(\))"

    # extract mult(#,#) integer pairs      
    matches_list = list()
    for text_string in data:
        matches = re.finditer(pattern, text_string)
        matches_list.append(matches)

    # combine with do() and don't() instructions
    flattened_matches_list = list()
    for match in list(chain.from_iterable(matches_list)):
        first_mult = (int(match.group(1)), int(match.group(2)))
        in_between = match.group(3)
        found_patterns = re.findall(subpattern, in_between)
        flattened_matches_list.append((first_mult, found_patterns))
    #print(flattened_matches_list)
   
    # q1: compute partial result
    result = 0
    for match in flattened_matches_list:
        mult = match[0]  # multiply statement
        result += mult[0] * mult[1]
    
    print(f"q1 result: {result}")

    # q2: add switch commands
    result = 0
    switch = True
    for match in flattened_matches_list:
        mult, cmds = match
        #print(switch, mult, cmds)
        
        if switch:
            result += mult[0] * mult[1]

        for cmd in cmds:
            if cmd == "do()":
                switch = True
            elif cmd == "don't()":
                switch = False
            else:
                ValueError(f"Command {cmd} not recognized")
    
    print(f"q2 result: {result}")
