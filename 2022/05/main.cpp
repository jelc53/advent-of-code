#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <tuple>

#include "funcs.hpp"

int main(int argc, char *argv[]) {

    // check command line args
    if (argc <= 1) {
        std::cout << "Usage:" << std::endl;
        std::cout << " $ ./main <input data file>" << std::endl;
    }

    // read input data
    std::string filename = argv[1];
    std::ifstream infile;

    std::vector<std::string> data;
    std::string line;

    infile.open(filename);
    if (infile.is_open()) {
        while (std::getline(infile, line)) {
            data.push_back(line);
        }
        infile.close();
    }

    // find dividing line
    int divide_idx = find_dividing_line(data);
    std::string stack_numbers = data[divide_idx - 1];

    // organize stack information
    std::vector<std::stack<char>> stacks;
    stacks = organize_stack_information(data, stack_numbers, divide_idx);

    // execute movements
    update_stack_placements(data, stacks, divide_idx);

    // print results
    for (auto i: stacks) {std::cout << i.top() << std::endl; }

    return 0;
}
