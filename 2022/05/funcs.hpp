#ifndef FUNCS_HPP
#define FUNCS_HPP

#include <vector>
#include <string>
#include <stack>

int find_dividing_line(std::vector<std::string> data);

std::vector<std::stack<char>> organize_stack_information(
        std::vector<std::string> data,
        std::string stack_number_line,
        int divide_idx
);

void update_stack_placements(
        std::vector<std::string> data,
        std::vector<std::stack<char>>& stacks,
        int divide_idx
);

#endif /* funcs.hpp */
