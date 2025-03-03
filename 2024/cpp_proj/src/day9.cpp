#include <fstream>
#include <iostream>
#include <string>

const char ZERO = '0';

char to_char(unsigned num) { return num + ZERO; }
unsigned to_num(char c) { return c - ZERO; }

class DiskMap {
  /* The DiskMap class contains a representation of a disk with file space and
   * free space. Files and space are represented by a list of integers with the
   * following spec:
   * - Including the 0th index, even-index integers represent the number of
   * blocks in a file, and odd-index integers represent the number of free
   * blocks.
   */
private:
  std::vector<unsigned> raw_disk;

public:
  DiskMap(std::ifstream &file) {
    char c;
    while (file >> c) {
      raw_disk.push_back(to_num(c));
    }
  }
  friend std::ostream &operator<<(std::ostream &os, const DiskMap &disk);
};

std::ostream &operator<<(std::ostream &os, const DiskMap &disk) {
  // Print the disk map as an expanded representation, with each file expanded
  // into its ID (repeated the number of blocks times), and free space (repeated
  // as dots)
  os << "Disk Map: ";
  unsigned file_id = 0;
  for (size_t i = 0; i < disk.raw_disk.size(); i++) {
    if (i & (unsigned)0x1) { // Odd index, represents free space.
      os << std::string(disk.raw_disk[i], '.');
    } else { // Even index, represents file space.
      os << std::string(disk.raw_disk[i], to_char(file_id++));
    }
  }
  os << std::endl;
  return os;
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    std::cerr << "Usage: " << argv[0] << " <input_file>" << std::endl;
    return 1;
  }

  std::ifstream file(argv[1]);
  if (!file.is_open()) {
    std::cerr << "File " << argv[1] << " cannot be opened." << std::endl;
    return 1;
  }

  DiskMap disk(file);
  std::cout << disk << std::endl;
  return 0;
}
