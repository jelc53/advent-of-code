#ifndef FUNCS_HPP
#define FUNCS_HPP

#include<vector>
#include<string>

std::vector<std::vector<int>> readFile(const std::string& fileName);

int numSafeReports(
  const std::vector<std::vector<int>>& data,
  const bool& dampenerFlag
);


#endif /* funcs.hpp */
