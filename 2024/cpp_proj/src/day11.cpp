#include <cstddef>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <list>
#include <string>

std::list<uint64_t> build_initial_list(std::string input) {
  std::list<uint64_t> ret;
  size_t idx;
  while (input.length() > 0) {
    idx = input.find(' ');
    if (idx != std::string::npos) {
      // Split the string into two parts: the non-whitespace before 'idx'
      // and the remainder after. Convert the part before to an unsigned integer
      // to add to the initial list.
      std::string next_integer = input.substr(0, idx);
      ret.emplace_back(std::move(std::stoull(next_integer)));
      input = input.substr(idx + 1);
    } else {
      // The final integer is just the entire string.
      ret.emplace_back(std::move(std::stoull(input)));
      input = input.substr(input.length());
    }
  }
  return ret;
}

void print_stones(std::list<uint64_t> &stones) {
  std::cout << "[ ";
  for (auto &num : stones) {
    std::cout << num << ", ";
  }
  std::cout << "]" << std::endl;
}

void apply_blinks(std::list<uint64_t> &stones, unsigned num_blinks) {
  for (size_t i = 0; i < num_blinks; i++) {
    std::cout << "Applying rules after blink number: " << i + 1 << std::endl;
    // Iterate over the stone list and apply the rules in order.
    for (std::list<uint64_t>::iterator it = stones.begin(); it != stones.end();
         it++) {
      // Rule 1: A zero stone becomes 1.
      if (*it == 0) {
        *it = 1;
        continue;
      }
      // Rule 2: A stone with an even number of digits becomes 2 stones, each
      // with half the digits from the original.
      std::string s = std::to_string(*it);
      if ((s.length() & 0x1) == 0) {
        size_t midpoint = s.length() / 2;
        std::string left_half = s.substr(0, midpoint);
        std::string right_half = s.substr(midpoint); // by default goes to end

        // Set the current stone position to the right half, and insert the
        // second stone into the list. Do this because the insert() function in
        // the list c++ library says the insertion happens _before_ the provided
        // iterator.
        *it = std::stoull(right_half);
        stones.insert(it, std::stoull(left_half));
        continue;
      }
      // Rule 3: Multiply the stones value by 2024.
      (*it) *= 2024;
    }
    // print_stones(stones);
  }
}

int main(int argc, char *argv[]) {
  if (argc != 3) {
    std::cerr << "Usage: " << argv[0] << " <input_file> <num_blinks>"
              << std::endl;
    return -1;
  }

  std::ifstream in;
  in.open(argv[1], std::ifstream::in);
  if (!in.is_open()) {
    std::cerr << "Failed to open file " << argv[1] << std::endl;
  }

  std::string line;
  if (!std::getline(in, line)) {
    std::cerr << "Failed to get input line from file, exiting!" << std::endl;
    return 1;
  }

  std::list<uint64_t> stones = build_initial_list(line);
  apply_blinks(stones, std::stoull(argv[2]));
  std::cout << "Final stone count: " << stones.size() << std::endl;
  return 0;
}
