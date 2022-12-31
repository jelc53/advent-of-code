import numpy as np
import sys
import os

def find_alignment_position1(positions):
    return np.median(positions)

def find_alignment_position2(positions):
    avg = np.mean(positions)
    return np.ceil(avg), np.floor(avg)

def calculate_fuel_to_align1(align_index, positions):
    fuel = 0.0
    for pos in positions:
        fuel += abs(pos - align_index)
    return fuel

def calculate_fuel_to_align2(align_index, positions):
    fuel = 0.0
    for pos in positions:
        n = abs(pos - align_index)
        fuel += n*(n+1)/2
    return fuel

def main():
    input_filename = sys.argv[1]
    data_path = os.path.join('07', input_filename)
    with open(data_path, 'r') as f:
        positions = f.read().strip().split(',')
        positions = np.array(list(map(int, positions)))
        
    # part 1: simple fuel usage
    align_index = find_alignment_position1(positions)
    fuel1 = calculate_fuel_to_align1(align_index, positions)
    print("Fuel required to align (simple): {}".format(fuel1))

    # part 2: increasing fuel usage
    align_index_ceil, align_index_floor = find_alignment_position2(positions)
    fuel2_ceil = calculate_fuel_to_align2(align_index_ceil, positions)
    fuel2_floor = calculate_fuel_to_align2(align_index_floor, positions)
    print("Fuel required to align (complex): {}".format(min(fuel2_ceil, fuel2_floor)))

if __name__ == "__main__":
    main()
