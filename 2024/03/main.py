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
    pattern = r"mul\((\d+),(\d+)\)"    
    matches_list = list()
    for text_string in data:
        matches = [(int(x), int(y)) for x, y in re.findall(pattern, text_string)]
        matches_list.append(matches)
    flattened_matches_list = list(chain.from_iterable(matches_list))

    # q1: compute partial result
    result = 0
    for mult in flattened_matches_list:
        result += mult[0] * mult[1]
    print(f"Partial result: {result}")
