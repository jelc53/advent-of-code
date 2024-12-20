#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>

std::vector<std::vector<int>> readFile(const std::string& fileName) {
  std::ifstream file(fileName);  // open file
  std::vector<std::vector<int>> lines;  // vector of vectors

  if (!file.is_open()) {
    std::cerr << "Error: could not open file " << fileName << std::endl;
    return lines;
  }

  std::string line;
  while (std::getline(file, line)) {
    std::istringstream iss(line);  // string stream
    std::vector<int> report;
    int num;

    while (iss >> num) {
      report.push_back(num);
    }

    lines.push_back(report);
  }

  file.close();
  return lines;
}

int numSafeReports(std::vector<std::vector<int>> data) {
  int numSafe = 0;
  for (size_t i = 0; i < data.size(); ++i) {
    std::vector<int> diffs;
    for (size_t j = 0; j < (data[i].size()-1); ++j) {
      diffs.push_back(data[i][j] - data[i][j+1]);
    }
    // check montonicity
    if (i == 1) {
    for (size_t k = 0; diffs.size(); ++k) {
        std::cout << diffs[k] << std::endl;     
      }     
    }
  }
  return numSafe;
}

int main(int argc, char *argv[]) {
  if (argc <= 1) {
    // no arguments, print useage message
    std::cout << "Usage: " <<std::endl;
    std::cout << "$ ./main <input data file>" << std::endl;
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
  int num_safe = numSafeReports(data);
  std:: cout << "Num safe reports: " << num_safe << std::endl;

  return 0;
}
