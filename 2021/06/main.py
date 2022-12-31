from collections import Counter
import os

def part1(fishes):
    for day in range(80):
        for idx, fish in enumerate(fishes):
            if fish == 0:
                fishes[idx] = 7
                fishes.append(9)
        fishes = [fish - 1 for fish in fishes]
    return len(fishes)

def part2(fishes):
    timers = Counter({timer: 0 for timer in range(10)})
    fishes = Counter(fishes)
    fishes.update(timers)
    print(fishes)
    for day in range(256):
        fishes[7] += fishes.get(0, 0)
        fishes[9] += fishes.get(0, 0)
        fishes = {fish: fishes.get(fish + 1, 0) for fish in fishes}

    return sum(fishes.values())

def main():
    with open(os.path.join("06", "data.txt"), 'r') as f:
        fishes = f.read().strip().split(',')
        fishes = list(map(int, fishes))
        print(fishes)

    print(f"Part 1 Answer: {part1(fishes)}")
    print(f"Part 2 Answer: {part2(fishes)}")

if __name__ == "__main__":
    main()
