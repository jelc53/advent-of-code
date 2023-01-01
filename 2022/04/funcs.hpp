#ifndef FUNCS_HPP
#define FUNCS_HPP

#include <string>
#include <vector>
#include <tuple>

int find_contained_pairs(
        std::vector<std::tuple<int, int>> a_vec,
        std::vector<std::tuple<int, int>> b_vec
);

int find_overlap_pairs(
        std::vector<std::tuple<int, int>> a_vec,
        std::vector<std::tuple<int, int>> b_vec
);

#endif /* FUNCS_HPP */
