#include <stack>
#include <string>
#include <vector>
#include <sstream>
#include <iostream>

#include "funcs.hpp"


int find_dividing_line(std::vector<std::string> data) {
    int idx;
    for (uint i = 0; i < data.size(); i++) {
        if (data[i].length() == 0) {
            idx = i;
            break;
        }
    }
    return idx;
}


std::vector<std::stack<char>> organize_stack_information(
        std::vector<std::string> data,
        std::string stack_number_line,
        int divide_idx
) {
    std::vector<std::stack<char>> stacks;

    for (uint i = 0; i < stack_number_line.size(); i++) {
        if (stack_number_line[i] == ' ') {
            continue;
        }
        std::stack<char> column_stack;

        for (int j = divide_idx - 2; j >= 0; j--) {
            if (data[j][i] == ' ') {
                break;
            }
            column_stack.push(data[j][i]);
        }

        stacks.push_back(column_stack);
    }

    return stacks;
}


void execute_block_instruction(
        std::vector<std::stack<char>>& stacks,
        int amount_to_move,
        int source_stack,
        int dest_stack
) {
    std::stack<char> helper_stack = std::stack<char>();
    for (int s = 0; s < amount_to_move; s++) {
        char ch = stacks[source_stack].top();
        stacks[source_stack].pop();
        helper_stack.push(ch);
    }
    while (!helper_stack.empty()) {
        char ch = helper_stack.top();
        helper_stack.pop();
        stacks[dest_stack].push(ch);
    }
}


void execute_move_instruction(
        std::vector<std::stack<char>>& stacks,
        int amount_to_move,
        int source_stack,
        int dest_stack
) {
    for (int s = 0; s < amount_to_move; s++) {
        char ch = stacks[source_stack].top();
        //std::cout << "moving " << ch << " from stack " << source_stack << " to stack " << dest_stack << std::endl;

        stacks[source_stack].pop();
        stacks[dest_stack].push(ch);
    }
}


void update_stack_placements(
        std::vector<std::string> data,
        std::vector<std::stack<char>>& stacks,
        int divide_idx
 ) {

    for (uint i = divide_idx + 1; i < data.size(); ++i) {

        std::vector<std::string> tokens;
        std::string token;
        std::stringstream ss(data[i]);

        while (std::getline(ss, token, ' ')) {
            tokens.push_back(token);
        }

        int amount_to_move = std::stoi(tokens[1]);
        int source_stack = std::stoi(tokens[3]) - 1;
        int dest_stack = std::stoi(tokens[5]) - 1;  // zero-based indexing

        //execute_move_instruction(stacks, amount_to_move, source_stack, dest_stack);
        execute_block_instruction(stacks, amount_to_move, source_stack, dest_stack);
    }
}


