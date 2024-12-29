#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
#include <algorithm>
#include <numeric>

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

bool isMonotonicIncreasing(const std::vector<int>& vec) {
    return std::adjacent_find(vec.begin(), vec.end(), std::greater<int>()) == vec.end();
}

bool isMonotonicDecreasing(const std::vector<int>& vec) {
    return std::adjacent_find(vec.begin(), vec.end(), std::less<int>()) == vec.end();
}

bool isStrictlyMonotonic(const std::vector<int>& vec) {
  bool increasing = true;
  bool decreasing = true;

  for (size_t i = 1; i < vec.size(); ++i) {
    if (vec[i] > vec[i - 1]) {
      decreasing = false;
    } else if (vec[i] - vec[i - 1]) {
      increasing = false;
    } else {
      return false;
    }
  }
  
  return increasing || decreasing;
}

bool levelsCheck(const std::vector<int>& vec) {
    for (size_t k = 0; k < vec.size() - 1; ++k) {
        int delta = std::abs(vec[k] - vec[k + 1]);
        if (delta < 1 || delta > 3) {
            return false;
        }
    }
    return true;
}

int numSafeReports(const std::vector<std::vector<int>>& data) {
  int numSafe = 0;
  
  for (const auto& report : data) {
    
    // check montonicity and level constraints
    if (isStrictlyMonotonic(report) && levelsCheck(report)) {
      numSafe++;
    }
  }
  return numSafe;
}

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
  int num_safe = numSafeReports(data);
  std:: cout << "Num safe reports: " << num_safe << std::endl;

  return 0;
}
