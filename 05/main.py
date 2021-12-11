import numpy as np
import sys
import os

def load_data(data_path):
    """ xx """
    max_idx = 0.0
    data = list()
    with open(data_path, 'r') as f:

        for line in f.readlines():
            line_segments = line.split('->')
            start_coord = line_segments[0].split(',')
            end_coord = line_segments[1].split(',')
            x1 = int(start_coord[0].strip())
            y1 = int(start_coord[1].strip())
            x2 = int(end_coord[0].strip())
            y2 = int(end_coord[1].strip())
            data.append([(x1, y1), (x2, y2)])

            if max([x1, y1, x2, y2]) >= max_idx:
                max_idx = max([x1, y1, x2, y2])

    return data, max_idx+1

def create_cartesian_map(data, max_idx):
    """ xx """
    # set dimensions
    cartesian_map = np.zeros((int(max_idx), int(max_idx)))

    # loop through each coord pair
    for line in data:
        first_coord = line[0]
        second_coord = line[1]
        x_min = min(first_coord[0], second_coord[0])
        x_max = max(first_coord[0], second_coord[0])
        y_min = min(first_coord[1], second_coord[1])
        y_max = max(first_coord[1], second_coord[1])

        # case 1: vertical line
        if x_min == x_max:
            while y_min <= y_max:
                cartesian_map[y_min][first_coord[0]] += 1
                y_min += 1

        # case 2: horizontal line
        elif y_min == y_max:
            while x_min <= x_max:
                cartesian_map[first_coord[1]][x_min] += 1
                x_min += 1

        # case 3: diagonal line
        else:
            x_delta = first_coord[0] - second_coord[0]
            y_delta = first_coord[1] - second_coord[1]

            # coords move in same direction
            if x_delta * y_delta >= 0:
                while x_min <= x_max:
                    cartesian_map[y_min][x_min] += 1
                    x_min += 1
                    y_min += 1

            # coords move in opposite direction
            else:
                while x_min <= x_max:
                    cartesian_map[y_max][x_min] += 1
                    x_min += 1
                    y_max -= 1

    return cartesian_map

def find_at_least_n_overlaps(cartesian_map, n=2):
    """ xx """
    cartesian_map = cartesian_map.astype(int)
    value_counts = np.bincount(cartesian_map.flatten())
    score = sum(value_counts[n:])

    return score

if __name__ == '__main__':

    input_filename = sys.argv[1]
    data_path = os.path.join('05', input_filename)
    data, max_idx = load_data(data_path)
    cartesian_map = create_cartesian_map(data, max_idx)
    score = find_at_least_n_overlaps(cartesian_map, n=2)
    print("Number of intersections: {}".format(score))
