#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <tuple>
#include <map>

int first_puzzle (std::vector<std::tuple<char, char>> data) {
	std::map<char, int> map1({{'A', 1}, {'B', 2}, {'C', 3}});
	std::map<char, int> map2({{'X', 1}, {'Y', 2}, {'Z', 3}});

	int score = 0;
	char p1, p2;

	for (uint i = 0; i < data.size(); i++) {
		p1 = std::get<0>(data[i]);
		p2 = std::get<1>(data[i]);

		score += map2[p2];

		if (map1[p1] == map2[p2]) {
			score += 3;
		}
		else if ((p1 == 'A' && p2 == 'Y') ||
				 (p1 == 'B' && p2 == 'Z') ||
				 (p1 == 'C' && p2 == 'X')) {
			score += 6;
		}
	}
	return score;
}

int second_puzzle (std::vector<std::tuple<char, char>> data) {
	std::map<char, int> value_map({{'A', 1}, {'B', 2}, {'C', 3}});
	std::map<char, int> result_map({{'X', 0}, {'Y', 3}, {'Z', 6}});

	int score = 0;
	char p1, x2;

	for (uint i = 0; i < data.size(); i++) {
		p1 = std::get<0>(data[i]);
		x2 = std::get<1>(data[i]);

		score += result_map[x2];
		int p1_val = value_map[p1];

		if (x2 == 'Y') {
			score += value_map[p1];
		}
		else if (x2 == 'Z') {
			if (p1_val + 1 <= 3){
				score += p1_val + 1;
			}
			else {
				score += 1;
			}
		}
		else if (x2 == 'X') {
			if (p1_val - 1 > 0) {
				score += p1_val - 1;
			}
			else {
				score += 3;
			}
		}
	}
	return score;
}

int main(int argc, char *argv[]) {
    if (argc <= 1) {
        // no arguments, print usage message
        std::cout << "Usage:" << std::endl;
        std::cout << " $ ./main <input data file>" << std::endl;
    }

	// declare filename variable
    std::string infile_name = argv[1];
    std::ifstream infile_stream;

    // read data file
    std::vector<std::tuple<char, char>> data;
    infile_stream.open("data.txt");

    if (infile_stream.is_open()) {
        char i, j;
        while (infile_stream >> i >> j) { // stream extraction operator
			data.emplace_back(i, j);
        }
        infile_stream.close();
    }
    else {
        std::cerr << "ERROR : Failed to open data file" << std::endl;
    }

    //for(auto& tuple: data) {
    //    std::cout << std::get<0>(tuple) << " " << std::get<1>(tuple) << std::endl;
    //}
	//std::cout << data.size() << std ::endl;

	// solve the puzzles
	int s1 = first_puzzle(data);
	std::cout << "first puzzle result: " << s1 << std::endl;

	int s2 = second_puzzle(data);
	std::cout << "second puzzle result: " << s2 << std::endl;

    return 0;
}
