import os
import sys

if __name__ == '__main__':
    count = 0
    data = list()
    infile = sys.argv[1]
    with open(os.path.join('2022', '01', infile), 'r') as f:
        for value in f.readlines():
            if value == '\n':  # separator between elves
                data.append(count)
                count = 0
            else:
                count += int(value)

    print('Elf carrying most calories has {} calories in his bag'.format(max(data)))

    top3 = sum(sorted(data, reverse=True)[:3])
    print('Top 3 elves are carrying {} calories across their bags'.format(top3))
