#include <string>
#include <vector>
#include <iostream>

#include "funcs.hpp"

char find_common(std::string s1, std::string s2) {
    for (uint j = 0; j < s1.length(); j++) {
        for (uint k = 0; k < s2.length(); k++) {
            if (s1[j] == s2[k]) {
                return s1[j];
            }
        }
    }
    return 0;
}

char find_common_three(std::string s1, std::string s2, std::string s3) {
    for (uint i = 0; i < s1.length(); i++) {
        for (uint j = 0; j < s2.length(); j++) {
            if (s1[i] == s2[j]) {
                for (uint k = 0; k < s3.length(); k++) {
                    if (s1[i] == s3[k]) {
                        return s1[i];
                    }
                }
            }
        }
    }
    return 0;
}

std::vector<char> common_characters(std::vector<std::string> data) {

    std::vector<char> out_vec;

    for (uint i = 0; i < data.size(); i++) {
        std::string s = data[i];
        long unsigned int len = s.length();
        std::string s1 = s.substr(0, len/2);
        std::string s2 = s.substr(len/2);
        out_vec.push_back(find_common(s1, s2));

    }
    return out_vec;
}

int convert_to_value(std::vector<char> common_vec) {

    int out_val = 0;

    for (uint i = 0; i < common_vec.size(); i++) {
        char c = common_vec[i];
        int cint = (int)(c);

        if (cint > 90)  { out_val += cint-96; }
        else { out_val += cint-(65-27); }
    }

    return out_val;
}

std::vector<char> identify_badges(std::vector<std::string> data) {

    std::vector<char> out_vec;

    for (uint i = 0; i < data.size(); i+=3) {
        std::string s1 = data[i];
        std::string s2 = data[i+1];
        std::string s3 = data[i+2];
        out_vec.push_back(find_common_three(s1, s2, s3));
    }
   // for (auto it = data.begin(); it != data.end(); it+3) {
   //     std::string s1 = *it;
   //     std::string s2 = *(it+1);
   //     std::string s3 = *(it+2);
   //     std::cout << s1 << s2 << s3 << std::endl;
   //     out_vec.push_back(find_common_three(s1, s2, s3));
   // }
    return out_vec;
}

