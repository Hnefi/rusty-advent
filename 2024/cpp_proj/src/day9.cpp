#include <fstream>
#include <iostream>
#include <vector>
#include <xtensor/xadapt.hpp>
#include <xtensor/xarray.hpp>
#include <xtensor/xbuilder.hpp>
#include <xtensor/xindex_view.hpp>
#include <xtensor/xio.hpp>
#include <xtensor/xmath.hpp>
#include <xtensor/xtensor_forward.hpp>
#include <xtensor/xutils.hpp>
#include <xtensor/xview.hpp>

const char ZERO = '0';
const char FREE_CHAR = '.';

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
  std::vector<char> raw_disk;
  xt::xarray<char> vec_disk;

public:
  DiskMap(std::ifstream &file) {
    char c;
    unsigned file_id = 0;
    bool is_file = true;
    while (file >> c) {
      while (to_num(c--) > 0) {
        if (is_file) {
          raw_disk.push_back(to_char(file_id));
        } else {
          raw_disk.push_back('.');
        }
      }
      if (is_file) {
        file_id++;
      }
      is_file = !is_file;
    }

    // Adapt the utility parsed disk var into the vectorized format.
    vec_disk = xt::adapt(raw_disk);
  }
  friend std::ostream &operator<<(std::ostream &os, const DiskMap &disk);

  void compact() {
    // Compact the disk by doing the following:
    // - build another vector whose values represent the INDEX of the final
    // block that should
    //   be placed at this location in the compacted disk.
    // e.g., for a basic disk 0...1.222, we would generate
    //                        087645123
    //       - note that the "8" represents the last "2" coming into the 1st
    //       position
    //       - and so on and so forth with the 7 6.
    //       - The 4 means that the 1 does not change its position
    //       - The 5 means that the empty block won't be filled.
    // - Then, the final compacted disk is simply going to be: final[i] =
    // initial[index_swaps[i]]
    //   which should be trivially vectorizable.
    xt::xarray<size_t> index_swaps = xt::arange<size_t>(0, vec_disk.size(), 1);
    size_t free_idx = 0, file_idx = vec_disk.size() - 1;
    while (free_idx < file_idx) {
      // std::cout << *this << "free_idx: " << free_idx
      //           << ", file_idx: " << file_idx << std::endl;
      if (vec_disk[free_idx] != FREE_CHAR) {
        // std::cout << "Found file block: " << vec_disk[free_idx] << std::endl;
        index_swaps[free_idx++] = free_idx;
      } else {
        // std::cout << "Found free block to compact at index " << free_idx
        //           << std::endl;
        while (vec_disk[file_idx] == FREE_CHAR) {
          // std::cout << "Advancing file_block pointer looking for a file block
          // "
          //              "to compact... file_idx = "
          // << file_idx << std::endl;
          index_swaps[file_idx--] = file_idx;
        }
        // std::cout << "Swapping indices " << free_idx << " and " << file_idx
        //           << std::endl;
        // Getting here means that file_idx points to a non-free space block
        // that we can swap into the place pointed to by 'free_idx'.
        index_swaps[free_idx] = file_idx;
        index_swaps[file_idx] = free_idx;
        free_idx++;
        file_idx--;
        // std::cout << "Index swaps xarray: " << index_swaps << std::endl;
      }
    }
    auto compacted = xt::index_view(vec_disk, index_swaps);
    // std::cout << "Final compacted view: " << compacted << std::endl;
    // Calculate the disk checksum.
    // xt::xarray<size_t> indices = xt::arange<size_t>(0, vec_disk.size(), 1);
    // auto product = (compacted - ZERO);
    // std::cout << "Final product: " << product << std::endl;
    unsigned long idx = 0;
    unsigned long long checksum = 0;
    for (auto &c : compacted) {
      if (c != FREE_CHAR) {
        checksum += (to_num(c) * idx++);
      }
    }
    std::cout << "Final checksum: " << checksum << std::endl;
  }
};

std::ostream &operator<<(std::ostream &os, const DiskMap &disk) {
  // Print the disk map as an expanded representation, with each file expanded
  // into its ID (repeated the number of blocks times), and free space (repeated
  // as dots)
  os << "Disk Map: ";
  for (auto &c : disk.vec_disk) {
    os << c;
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
  disk.compact();
  return 0;
}
