#include <algorithm>
#include <cctype>
#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <functional>
#include <iostream>
#include <unordered_set>
#include <xtensor/xadapt.hpp>
#include <xtensor/xarray.hpp>
#include <xtensor/xbuffer_adaptor.hpp>
#include <xtensor/xio.hpp>
#include <xtensor/xlayout.hpp>
#include <xtensor/xoperation.hpp>
#include <xtensor/xstrided_view.hpp>
#include <xtensor/xtensor_forward.hpp>
#include <xtensor/xview.hpp>

#define PART_TWO

typedef std::pair<size_t, size_t> _AntennaCoordinates;

const char ANTINODE_CHAR = '#';

class AntennaPair {
  // A class representing a pair of antennas in a 2D city map.
private:
  _AntennaCoordinates antenna1;
  _AntennaCoordinates antenna2;

public:
  AntennaPair(_AntennaCoordinates antenna1, _AntennaCoordinates antenna2)
      : antenna1(antenna1), antenna2(antenna2) {}
  _AntennaCoordinates get_antenna1() const { return this->antenna1; }
  _AntennaCoordinates get_antenna2() const { return this->antenna2; }
  void set_antenna1(_AntennaCoordinates antenna1) { this->antenna1 = antenna1; }
  void set_antenna2(_AntennaCoordinates antenna2) { this->antenna2 = antenna2; }

  bool operator==(const AntennaPair &other) const {
    return this->antenna1 == other.antenna1 && this->antenna2 == other.antenna2;
  }

  void order_antennas() {
    // Order the antennas in their respective coordinates so that antenna1 is
    // always the one with the smaller row index, and if the row indices are the
    // same, the one with the smaller column index.
    if ((this->antenna1.first < this->antenna2.first) ||
        ((this->antenna1.first == this->antenna2.first) &&
         this->antenna1.second < this->antenna2.second)) {
      return;
    } else {
      _AntennaCoordinates temp = this->antenna1;
      this->antenna1 = this->antenna2;
      this->antenna2 = temp;
    }
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
  // std::unordered_set<char> chars_tested;

  for (size_t i = 0; i < city_map.shape()[0]; i++) {
    for (size_t j = 0; j < city_map.shape()[1]; j++) {
      const char &char_to_test = city_map(i, j);
      if (std::isalnum(char_to_test)) {
        // std::cout << "Found antenna at (" << i << ", " << j
        //           << "): " << char_to_test << std::endl;
        // Create a new 2D array with the same dimensions as the city map
        // comprised copmletely of the found antenna character, to vector
        // search all the other antennas with one expression.
        auto antenna_char_search = xt::full_like(city_map, char_to_test);
        auto antenna_mask = xt::equal(city_map, antenna_char_search);
        auto indices_of_pairs = xt::argwhere(antenna_mask);
        for (auto &index : indices_of_pairs) {
          if (index[0] != i && index[1] != j) {
            // Only add non-redundant pairs.
            AntennaPair found_pair =
                AntennaPair(_AntennaCoordinates(i, j),
                            _AntennaCoordinates(index[0], index[1]));
            found_pair.order_antennas();
            antenna_pairs.insert(found_pair);
            // std::cout << "Found pair: (" << i << ", " << j << ") and ("
            //           << index[0] << ", " << index[1] << ")" << std::endl;
          }
        }
      }
    }
  }
  return antenna_pairs;
}

void place_antinodes(xt::xarray<char, xt::layout_type::row_major> &city_map,
                     const std::unordered_set<AntennaPair> &antenna_pairs) {
  // Place the antinodes in the city map, where an antinode is represented by
  // the character 'X'. An antinode can overwrite an antenna character, because
  // we have already determined the AntennaPair locations previously.
  for (const AntennaPair &pair : antenna_pairs) {
    _AntennaCoordinates antenna1 = pair.get_antenna1();
    _AntennaCoordinates antenna2 = pair.get_antenna2();
#ifdef PART_TWO
    // All antennae themselves are antinodes.
    city_map[{antenna1.first, antenna1.second}] = ANTINODE_CHAR;
    city_map[{antenna2.first, antenna2.second}] = ANTINODE_CHAR;
#endif
    // Place two antinodes by finding the vertical and horizontal strides
    // between the two antennas, and adding/subtracting them from the indices of
    // the two antennas.
    int vertical_stride = antenna2.first - antenna1.first;
    int horizontal_stride = antenna2.second - antenna1.second;

#ifdef PART_TWO
    // For resonant frequencies, loop until the antinodes are out of bounds.
    while (city_map.in_bounds(antenna1.first - vertical_stride,
                              antenna1.second - horizontal_stride)) {
      antenna1.first -= vertical_stride;
      antenna1.second -= horizontal_stride;
      city_map[{antenna1.first, antenna1.second}] = ANTINODE_CHAR;
    }
    while (city_map.in_bounds(antenna2.first + vertical_stride,
                              antenna2.second + horizontal_stride)) {
      // std::cout << "Placing antinode at (" << second_antinode_x << ", "
      //           << second_antinode_y << ")" << std::endl;
      antenna2.first += vertical_stride;
      antenna2.second += horizontal_stride;
      city_map[{antenna2.first, antenna2.second}] = ANTINODE_CHAR;
    }
#else
    int first_antinode_x = antenna1.first - vertical_stride;
    int first_antinode_y = antenna1.second - horizontal_stride;
    if (city_map.in_bounds(first_antinode_x, first_antinode_y)) {
      // std::cout << "Placing antinode at (" << first_antinode_x << ", "
      //           << first_antinode_y << ")" << std::endl;
      city_map[{first_antinode_x, first_antinode_y}] = ANTINODE_CHAR;
    }
    int second_antinode_x = antenna2.first + vertical_stride;
    int second_antinode_y = antenna2.second + horizontal_stride;
    if (city_map.in_bounds(second_antinode_x, second_antinode_y)) {
      // std::cout << "Placing antinode at (" << second_antinode_x << ", "
      //           << second_antinode_y << ")" << std::endl;
      city_map[{second_antinode_x, second_antinode_y}] = ANTINODE_CHAR;
    }
#endif
  }
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
  size_t shape_rows = 0, shape_cols = 0;
  finput.open(input_file, std::fstream::in);
  if (finput.is_open() && finput.good()) {
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

  std::cout << "Finding antennae..." << std::endl;
  auto pairs = find_antenna_pairs(city_map);
  std::cout << "Placing antinodes..." << std::endl;
  place_antinodes(city_map, pairs);
  // std::cout << city_map << std::endl;
  //  Find how many antinodes exist.
  size_t num_antinodes = 0;
  std::for_each(city_map.begin(), city_map.end(), [&num_antinodes](auto _char) {
    if (_char == ANTINODE_CHAR) {
      num_antinodes++;
    }
  });

  std::cout << "Number of antinodes: " << num_antinodes << std::endl;

  return 0;
}
