#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
#include <numeric>

#include "funcs.hpp"

int main(int argc, char *argv[]) {
  if (argc <= 1) {
    // no arguments, print useage message
    std::cout << "Usage: " <<std::endl;
    std::cout << "$ ./main <input data file>" << std::endl;
    return 1; // exit with error code
  }

  // read data file
  std::string fileName = argv[1];
  std::vector<std::vector<int>> data = readFile(fileName);

  // print first two rows (reports) of data
  for (size_t i = 0; i < 2; ++i) {
    std::cout << "Report " << i + 1 << ": ";
    for (int num : data[i]) {
      std::cout << num << " ";
    }
    std::cout << std::endl;
  }

  // q1: check report safety
  int num_safe = numSafeReports(data, false);
  std::cout << "Num safe reports: " << num_safe << std::endl;

  // q2: add problem dampener
  int num_safe_damp = numSafeReports(data, true);
  std::cout << "Num safe reports with dampener: " << num_safe_damp << std::endl;
   
  return 0;
}
