import sys
import os

def load_data(data_path):
    inputs, outputs = list(), list()
    with open(data_path, 'r') as f:
        for line in f.readlines():
            input, output = line.split('|')
            inputs.append(input.strip().split())
            outputs.append(output.strip().split())
    return inputs, outputs

def count_strings_of_specified_length(data, n):
    count = 0
    for line in data:
        for word in line:
            if len(word) == n:
                count += 1
    return count

def decode_input_strings(data):
    cipher = dict()
    unsolved = list()
    sorted_data = sorted(data, key=len)
    print(sorted_data)
    
    # a
    for i in sorted_data[1]:  # look at length 3
        if sorted_data[0].find(i) == -1:
            cipher['a'] = i
        else:
            unsolved.append(i)
    print(unsolved)
    
    # c, e
    for i in sorted_data[9]:  # look at length 7
        if sorted_data[8].find(i) == -1 or sorted_data[7].find(i) == -1 or sorted_data[6].find(i) == -1:
            if i in unsolved:
                cipher['c'] = i
                unsolved.remove(i)
            else:
                unsolved.append(i)
    print(unsolved)

    # f
    cipher['f'] = unsolved[0]
    unsolved.remove(unsolved[0])
    print(unsolved)

    # b
    for i in sorted_data[2]:  # look at length 4
        if i not in cipher.values():
            cipher['b'] = i
    
    print(cipher)


    return cipher

def main():
    input_filename = sys.argv[1]
    data_path = os.path.join('08', input_filename)
    input, output = load_data(data_path)

    # part 1: sum strings of length 2, 3, 4, 7
    unique_count = 0
    unique_lengths = [2, 3, 4, 7]
    for n in unique_lengths:
        unique_count += count_strings_of_specified_length(output, n)
    print("Count of unique length codes: {}".format(unique_count))

    # part 2: decode cypher and add outputs
    dummy = [
        'acedgfb', 
        'cdfbe',
        'gcdfa',
        'fbcad',
        'dab',
        'cefabd',
        'cdfgeb',
        'eafb',
        'cagedb',
        'ab']
    cipher = decode_input_strings(dummy)


if __name__ == '__main__':
    main()
