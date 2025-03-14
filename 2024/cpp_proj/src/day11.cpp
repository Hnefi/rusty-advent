#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <functional>
#include <iostream>
#include <list>
#include <string>
#include <tuple>
#include <unordered_map>
#include <utility>

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

typedef std::tuple<uint64_t, unsigned> _map_key_t;

struct CustomTupleHash {
  std::size_t operator()(const _map_key_t &k) const noexcept {
    std::size_t hash = std::hash<uint64_t>{}(std::get<0>(k));
    // this xor and shift are randomly chosen
    hash ^= std::hash<uint64_t>{}(std::get<1>(k) << 2);
    return hash;
  }
};

// A memoization map with the following structure:
// - Keys: a tuple of (initial value, num_blinks)
// - Values: The number of stones after applying all the blinks.
static uint64_t lookups = 0;
static uint64_t hits = 0;
typedef std::unordered_map<_map_key_t, uint64_t, CustomTupleHash> _memo_map_t;

std::size_t number_of_stones(uint64_t initial_number, unsigned num_blinks,
                             _memo_map_t &memo) {
  // Calculate the number of stones that will be generated after 'num_blinks'
  // iterations, starting from a single initial_number. Uses memoization to
  // store repeated results.
  if (num_blinks == 0)
    return 1;

  // Check for memoization.
  lookups += 1;
  const auto &it = memo.find(std::make_tuple(initial_number, num_blinks));
  if (it != memo.end()) {
    hits += 1;
    return it->second;
  }

  // We missed the memoization map, so go and apply blinks-1 to the stones that
  // will result from following the rules.
  // Rule 1: A zero stone becomes 1.
  uint64_t val;
  if (initial_number == 0) {
    val = number_of_stones(1, num_blinks - 1, memo);
    memo.insert(std::make_pair(std::make_tuple(1, num_blinks - 1), val));
    return val;
  }
  // Rule 2: A stone with an even number of digits becomes 2 stones, each
  // with half the digits from the original.
  std::string s = std::to_string(initial_number);
  if ((s.length() & 0x1) == 0) {
    size_t midpoint = s.length() / 2;
    std::string left_half = s.substr(0, midpoint);
    std::string right_half = s.substr(midpoint); // by default goes to end
    uint64_t left_num = std::stoull(left_half);
    uint64_t right_num = std::stoull(right_half);
    val = number_of_stones(left_num, num_blinks - 1, memo);
    memo.insert(std::make_pair(std::make_tuple(left_num, num_blinks - 1), val));
    uint64_t tmp = number_of_stones(right_num, num_blinks - 1, memo);
    memo.insert(
        std::make_pair(std::make_tuple(right_num, num_blinks - 1), tmp));
    return val + tmp;
  }
  // Rule 3: Multiply the value by 2024;
  val = number_of_stones(initial_number * 2024, num_blinks - 1, memo);
  memo.insert(std::make_pair(
      std::make_tuple(initial_number * 2024, num_blinks - 1), val));
  return val;
}

uint64_t apply_blinks_part_two(std::list<uint64_t> &stones,
                               unsigned num_blinks) {
  uint64_t result = 0;
  _memo_map_t memo;
  for (std::list<uint64_t>::iterator it = stones.begin(); it != stones.end();
       it++) {
    // Apply all the blinks to each stone individually and sum up the values,
    // since each blink does not swap the order of any stones.
    result += number_of_stones(*it, num_blinks, memo);
  }
  return result;
}

std::size_t apply_blinks_part_one(std::list<uint64_t> &stones,
                                  unsigned num_blinks) {
  for (size_t i = 0; i < num_blinks; i++) {
    // std::cout << "Applying rules after blink number: " << i + 1 << std::endl;
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
  return stones.size();
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
  uint64_t blinks = std::stoull(argv[2]);
  // uint64_t count = apply_blinks_part_one(stones, blinks);
  uint64_t count = apply_blinks_part_two(stones, blinks);
  std::cout << "Final stone count: " << count << std::endl;
  std::cout << "Memoization hit rate: " << (float)hits / (float)lookups
            << std::endl;
  return 0;
}
