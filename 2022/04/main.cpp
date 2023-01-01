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
    std::vector<std::tuple<int, int>> a_vec, b_vec;

    infile.open(filename);
    if (infile.is_open()) {
        std::string input;

        while (std::getline(infile, input)) {
            std::string a = input.substr(0, input.find(","));
            std::string b = input.substr(input.find(",")+1);

            int a1 = std::stoi(a.substr(0, a.find("-")));
            int a2 = std::stoi(a.substr(a.find("-")+1));
            int b1 = std::stoi(b.substr(0, b.find("-")));
            int b2 = std::stoi(b.substr(b.find("-")+1));

            a_vec.emplace_back(a1, a2);
            b_vec.emplace_back(b1, b2);
        }
        infile.close();
    }

    // solve first puzzle
    int num_contained = find_contained_pairs(a_vec, b_vec);
    std::cout << "Number of fully contained pairs: " << num_contained << std::endl;

    // solve second puzzle
    int num_overlaps = find_overlap_pairs(a_vec, b_vec);
    std::cout << "Number of overlapping pairs: " << num_overlaps << std::endl;

    // for (auto i: a_vec) {std::cout << std::get<0>(i) << std::get<1>(i) << std::endl; }
    return 0;
}
