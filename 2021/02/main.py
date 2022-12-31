import os
import sys

if __name__ == '__main__':

    # variable declarations
    input_filename = sys.argv[1]
    aim_measure = 0
    vertical_position = 0
    horizontal_position = 0

    with open(os.path.join('02', input_filename), 'r') as f:

        # read file line by line
        for line in f.readlines():
            instruction = line.split()
            direction = instruction[0].strip()
            value = int(instruction[1].strip())

            # populate data containers
            if direction == 'forward':
                horizontal_position += value
                vertical_position += aim_measure * value

            elif direction == 'down':
                # vertical_position += value
                aim_measure += value

            else:
                # vertical_position -= value
                aim_measure -= value

    print("Product of horizontal and vertical positions: {}".format(horizontal_position*vertical_position))
