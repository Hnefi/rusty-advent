#include <cctype>
#include <cstddef>
#include <fstream>
#include <functional>
#include <iostream>
#include <unordered_set>
#include <xtensor/xadapt.hpp>
#include <xtensor/xarray.hpp>
#include <xtensor/xbuffer_adaptor.hpp>
#include <xtensor/xio.hpp>
#include <xtensor/xlayout.hpp>
#include <xtensor/xstrided_view.hpp>
#include <xtensor/xview.hpp>

typedef std::pair<size_t, size_t> _AntennaCoordinates;

class AntennaPair {
  // A class representing a pair of antennas in a 2D city map.
private:
  _AntennaCoordinates antenna1;
  _AntennaCoordinates antenna2;

public:
  _AntennaCoordinates get_antenna1() const { return this->antenna1; }
  _AntennaCoordinates get_antenna2() const { return this->antenna2; }
  void set_antenna1(_AntennaCoordinates antenna1) { this->antenna1 = antenna1; }
  void set_antenna2(_AntennaCoordinates antenna2) { this->antenna2 = antenna2; }

  bool operator==(const AntennaPair &other) const {
    return this->antenna1 == other.antenna1 && this->antenna2 == other.antenna2;
  }
};

template <> struct std::hash<AntennaPair> {
  std::size_t operator()(const AntennaPair &pair) const {
    return std::hash<size_t>()(pair.get_antenna1().first) ^
           std::hash<size_t>()(pair.get_antenna1().second) ^
           std::hash<size_t>()(pair.get_antenna2().first) ^
           std::hash<size_t>()(pair.get_antenna2().second);
  }
};

std::unordered_set<AntennaPair>
find_antenna_pairs(const xt::xarray<char> &city_map) {
  // Find all pairs of antennas in the city map, where an antenna is represented
  // by any alphanumeric character.
  std::unordered_set<AntennaPair> antenna_pairs;
  std::unordered_set<char> chars_tested;

  for (size_t i = 0; i < city_map.shape()[0]; i++) {
    for (size_t j = 0; j < city_map.shape()[1]; j++) {
      const char &char_to_test = city_map(i, j);
      if (std::isalnum(char_to_test)) {
        std::pair<std::unordered_set<char>::iterator, bool> insertion_result =
            chars_tested.insert(char_to_test);
        if (insertion_result.second) {
          // New antenna character found, get all the pairs.
          std::cout << "Found antenna at (" << i << ", " << j
                    << "): " << char_to_test << std::endl;
          // Create a new 2D array with the same dimensions as the city map
          // comprised copmletely of the found antenna character, to vector
          // search all the other antennas with one expression.
          auto antenna_char_search = xt::full_like(city_map, char_to_test);
          auto antenna_mask = xt::equal(city_map, antenna_char_search);
          std::cout << "Mask map for antenna character: " << char_to_test
                    << std::endl
                    << antenna_mask << std::endl;
        }
      }
    }
  }
  return antenna_pairs;
}

int main(int argc, char *argv[]) {

  if (argc < 2) {
    std::cerr << "Usage: " << argv[0] << " <input_file>" << std::endl;
    return 1;
  }

  std::string input_file = argv[1];
  std::cout << "Parsing the map input..." << std::endl;
  std::fstream finput;

  std::vector<char> map_raw;
  size_t shape_rows, shape_cols;
  finput.open(input_file, std::fstream::in);
  if (finput.is_open() || finput.good()) {
    std::string line;
    while (std::getline(finput, line)) {
      shape_cols = line.length();
      for (char c : line) {
        map_raw.push_back(c);
      }
      shape_rows++;
    }
  }

  // Convert the raw map into a 2D xtensor array.
  xt::xarray<char, xt::layout_type::row_major> city_map =
      xt::adapt(map_raw, std::vector<size_t>{shape_rows, shape_cols});

  auto pairs = find_antenna_pairs(city_map);

  return 0;
}
