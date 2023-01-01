#include <string>
#include <vector>
#include <iostream>
#include <tuple>

#include "funcs.hpp"


int find_contained_pairs(
        std::vector<std::tuple<int, int>> a_vec,
        std::vector<std::tuple<int, int>> b_vec
) {
    int count = 0;

    for (uint i = 0; i < a_vec.size(); i++) {
        if (std::get<0>(a_vec[i]) >= std::get<0>(b_vec[i]) &&
                    std::get<1>(a_vec[i]) <= std::get<1>(b_vec[i])) {
            count++;
            continue;
        }
        else if (std::get<0>(a_vec[i]) <= std::get<0>(b_vec[i]) &&
                    std::get<1>(a_vec[i]) >= std::get<1>(b_vec[i])) {
            count++;
            continue;
        }

    }
    return count;
}


int find_overlap_pairs(
        std::vector<std::tuple<int, int>> a_vec,
        std::vector<std::tuple<int, int>> b_vec
) {
    int count = 0;

    for (uint i = 0; i < a_vec.size(); i++) {
        if (std::get<0>(a_vec[i]) >= std::get<0>(b_vec[i]) &&
                    std::get<1>(a_vec[i]) <= std::get<1>(b_vec[i])) {
            count++;
            continue;
        }
        else if (std::get<0>(a_vec[i]) <= std::get<0>(b_vec[i]) &&
                    std::get<1>(a_vec[i]) >= std::get<1>(b_vec[i])) {
            count++;
            continue;
        }
        else if (std::get<1>(a_vec[i]) >= std::get<0>(b_vec[i]) &&
                    std::get<0>(a_vec[i]) <= std::get<1>(b_vec[i])) {
           count++;
           continue;
        }
    }

    return count;
}

