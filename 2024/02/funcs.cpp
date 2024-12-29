#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
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

bool isSafe(const std::vector<int>& report) {
  return isStrictlyMonotonic(report) && levelsCheck(report);
}

bool isSafeWithDampener(const std::vector<int>& report) {
  if (isSafe(report)) {
    return true;  // already safe without removing level
  }

  // check if removing level makes report safe
  for (size_t i = 0; i < report.size(); ++i) {
    std::vector<int> modifiedReport = report;
    modifiedReport.erase(modifiedReport.begin() + i);

    if (isSafe(modifiedReport)) {
      return true;  // found a safe report
    }
  }

  return false;  // not safe even with dampener
}

int numSafeReports(const std::vector<std::vector<int>>& data, const bool& dampenerFlag) {
  int numSafe = 0;

  for (const auto& report : data) {

    // check montonicity and level constraints
    if (dampenerFlag) {
      if (isSafeWithDampener(report)) {
        numSafe++;
      }

    } else {
      if (isSafe(report)) {
        numSafe++;
      }
    }

  }
  return numSafe;
}
