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
            data.append(line.strip())

    # flatten data
    text_string = "".join(data)

    # match patterns
    mult_pattern = r"mul\((\d+),(\d+)\)"
    command_pattern = r"(do\(\)|don't\(\))"
    combined_pattern = fr"{mult_pattern}|{command_pattern}"

    # q1: compute partial result
    result = 0
    matches = re.finditer(mult_pattern, text_string)
    operations = [(int(match.group(1)), int(match.group(2))) for match in matches]
    for mult in operations:
        result += mult[0] * mult[1]
        
    print(f"q1 result: {result}")
    
    # q2: add switch commands
    result, switch = 0, True
    matches = re.finditer(combined_pattern, text_string)
    for match in matches:
        if match.group(1) and match.group(2):  # mul(x, y)
            if switch:
                x, y = int(match.group(1)), int(match.group(2))
                result += x * y
        elif match.group(3):  # do() or don't()
            if match.group(3) == "do()":
                switch = True
            elif match.group(3) == "don't()":
                switch = False
     
    print(f"q2 result: {result}")

