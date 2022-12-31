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
    
    # a
    for i in sorted_data[1]:  # look at length 3
        if sorted_data[0].find(i) == -1:
            cipher[i] = 'a'
        else:
            unsolved.append(i)

    # c, e
    for i in sorted_data[9]:  # look at length 7
        if sorted_data[8].find(i) == -1 or sorted_data[7].find(i) == -1 or sorted_data[6].find(i) == -1:
            if i in unsolved:
                cipher[i] = 'c'
                unsolved.remove(i)
            else:
                unsolved.append(i)

    # f
    cipher[unsolved[0]] = 'f'
    unsolved.remove(unsolved[0])

    # b, d
    for i in sorted_data[2]:  # look at length 4
        if i not in cipher.keys():
            if sorted_data[3].find(i) != -1 and sorted_data[4].find(i) != -1 and sorted_data[5].find(i) != -1:
                cipher[i] = 'd'
            else:
                cipher[i] = 'b'

        if i in unsolved:
            unsolved.remove(i)

    # e, g
    cipher[unsolved[0]] = 'e'
    for i in sorted_data[9]:
        if i not in cipher.keys():
            cipher[i] = 'g'
    print(cipher)

    return cipher

def convert_output_to_value(cipher, output):
    value_parts = list()
    mapping = {
        '0': 'abcefg',
        '1': 'cf',
        '2': 'acdeg',
        '3': 'acdfg',
        '4': 'bcdf',
        '5': 'abdfg',
        '6': 'abdefg',
        '7': 'acf',
        '8': 'abcdefg',
        '9': 'abcdfg',
    }
    for out_str in output:
 
        # decode
        out_str_decode = str()
        for letter in out_str:
            out_str_decode += cipher[letter]

        # convert to value
        for k, v in mapping.items():
            if sorted(out_str_decode) == sorted(v):
                value_parts.append(k)

    value = str()
    for val in value_parts:
        value += val

    return int(value)

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
    total_sum = 0
    for idx, line in enumerate(input):
        cipher = decode_input_strings(line)
        output_value = convert_output_to_value(cipher, output[idx])
        total_sum += output_value
    print("Total sum of decoded outputs: {}".format(total_sum))


if __name__ == '__main__':
    main()
