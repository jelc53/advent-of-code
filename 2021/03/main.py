import sys
import os

def convert_binary_to_decimal(binary):
    """ Note, binary must not have leading zero """
    decimal, i = 0, 0
    while(binary != 0):
        dec = binary % 10
        decimal += dec * pow(2, i)
        binary = binary//10
        i += 1
    return decimal

def inbuilt_binary_to_decimal(binary):
    """ Takes string input, leading zero okay """
    return int(binary, 2)

def compute_power_consumption(data):
    # find binary strings for gamma and epsilon
    counter = [0]*len(data[0])
    for bin in data:
        for i, c in enumerate(bin):
            if c == '1':
                counter[i] += 1
            elif c == '0':
                counter[i] -= 1
            else:
                print("ERROR: There exists some non-binary data")

    most_common, least_common = [], []
    for i, c in enumerate(counter):
        if c > 0:
            least_common.append('0')
            most_common.append('1')
        elif c < 0:
            least_common.append('1')
            most_common.append('0')
        else:
            print("ERROR: There exists tie breaker entries")

    epsilon_bin = str(''.join(least_common))
    gamma_bin = str(''.join(most_common))

    # convert binary into decimal
    epsilon = inbuilt_binary_to_decimal(epsilon_bin)
    gamma = inbuilt_binary_to_decimal(gamma_bin)

    # calculate power consumption
    power = epsilon * gamma

    # output results to console
    print("Epsilon rate: {}, {}".format(epsilon_bin, epsilon))
    print("Gamma rate: {}, {}".format(gamma_bin, gamma))
    print("Power consumption: {}".format(power))

    return None

def implement_criteria(data, position=0, criteria=1):
    # if criteria == 1:  # 1 := most common
    count = 0
    for i, bin in enumerate(data):
        if bin[position] == '1':
            count += 1
        elif bin[position] == '0':
            count -= 1
        else:
            print("ERROR: There exists some non-binary data")

    if criteria == 1:
        if count >= 0:
            filtered_data = [bin for bin in data if bin[position] == '1']
        else:
            filtered_data = [bin for bin in data if bin[position] == '0']
    else:
        if count >= 0:
            filtered_data = [bin for bin in data if bin[position] == '0']
        else:
            filtered_data = [bin for bin in data if bin[position] == '1']

    return filtered_data

def compute_life_support_rating(data, criteria=1):
    # oxygen ...
    position = 0
    o2_data = data.copy()
    while len(o2_data) > 1:
        o2_data = implement_criteria(
            data=o2_data,
            position=position,
            criteria=1)
        position += 1
    oxygen_bin = o2_data[0]

    # co2 ...
    position = 0
    co2_data = data.copy()
    while len(co2_data) > 1:
        co2_data = implement_criteria(
            data=co2_data,
            position=position,
            criteria=0)
        position += 1
    co2_bin = co2_data[0]

    # convert binary into decimal
    oxygen = inbuilt_binary_to_decimal(oxygen_bin)
    co2 = inbuilt_binary_to_decimal(co2_bin)

    # calculate life support
    life_support = oxygen * co2

    # output results to console
    print("Oxygen generator: {}, {}".format(oxygen_bin, oxygen))
    print("CO2 scrubber: {}, {}".format(co2_bin, co2))
    print("Life support rating: {}".format(life_support))

    return None

if __name__ == '__main__':

    # declare input args
    input_filename = sys.argv[1]
    path_to_file = os.path.join('03', input_filename)

    # read in data file
    data = list()
    with open(path_to_file, 'r') as f:
        for line in f.readlines():
            data.append(str(line.strip()))

    # part 1: compute power consumption
    compute_power_consumption(data)

    # part 2: compute life supprot rating
    compute_life_support_rating(data)
