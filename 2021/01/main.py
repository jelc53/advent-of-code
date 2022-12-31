import numpy as np
import sys
import os

def count_number_of_increases(vec):
    counter = 0
    for i in range(1,len(vec)):
        if vec[i] > vec[i-1]:
            counter += 1
    return counter

if __name__ == '__main__':
    data = list()
    input_filename = sys.argv[1]
    with open(os.path.join('01', input_filename), 'r') as f:
        for depth in f.readlines():
            data.append(int(depth.strip()))

    # find number of depth increases
    count = count_number_of_increases(data)
    print("Increased depth count: {}".format(count))

    # find three-measurement window increases
    window3 = np.convolve(data, np.ones(3, dtype=int), 'valid')
    count3 = count_number_of_increases(window3)
    print("Increased three-window depth count: {}".format(count3))
