#include <iostream>
#include <fstream>
#include <string>
#include <vector>

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

    infile.open(filename);
    if (infile.is_open()) {
        std::string input;
        while (infile >> input) {
            data.push_back(input);
        }
        infile.close();
    }

    // solve first puzzle
    std::vector<char> common_vec;
    common_vec = common_characters(data);
    int value1 = convert_to_value(common_vec);
    // for (auto i: common_vec) {std::cout << i << std::endl; }
    std::cout << "Sum of priorities of common items: " << value1 << std::endl;

    // solve second puzzle
    std::vector<char> badge_vec;
    badge_vec = identify_badges(data);
    int value2 = convert_to_value(badge_vec);
    std::cout << "Sum of priorities for elf badges: " << value2 << std::endl;

    return 0;
}
