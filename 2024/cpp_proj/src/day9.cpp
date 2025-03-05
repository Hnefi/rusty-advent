#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <queue>
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
const int FREE_INT = -1;
const unsigned MAX_FILE_SIZE = 9;

unsigned to_num(char c) { return c - ZERO; }

void print_map(xt::xarray<int> arr, size_t line_length = 20) {
  // Print the disk map as an expanded representation, with each file expanded
  // into its ID (repeated the number of blocks times), and free space (repeated
  // as dots)
  size_t l = line_length;
  for (auto &c : arr) {
    std::cout << c << ",";
    if (l-- == 0) {
      std::cout << std::endl;
      l = line_length;
    }
  }
  std::cout << std::endl;
}

class Block {
private:
  size_t _idx;
  size_t _size;

public:
  Block(size_t index, size_t size) : _idx(index), _size(size) {}

  size_t get_index() const { return _idx; }
  size_t get_size() const { return _size; }
  void set_index(size_t new_index) { _idx = new_index; }
  void set_size(size_t new_size) { _size = new_size; }
};

class File : public Block {
private:
  unsigned file_id;

public:
  File(size_t index, size_t size, unsigned _id)
      : Block(index, size), file_id(_id) {}

  unsigned get_file_id() const { return file_id; }
};

class FreeSpace : public Block {
public:
  FreeSpace(size_t index, size_t size) : Block(index, size) {}
};

inline bool operator<(const FreeSpace &lhs, const FreeSpace &rhs) {
  return lhs.get_index() < rhs.get_index();
}

inline bool operator>(const FreeSpace &lhs, const FreeSpace &rhs) {
  return (rhs < lhs);
}

typedef std::priority_queue<
    FreeSpace, std::vector<FreeSpace>,
    std::greater<FreeSpace>> // use greater because we want the smallest
                             // FreeSpace blocks at the top of the heap
    _free_heap_t;
class DiskMap {
  /* The DiskMap class contains a representation of a disk with file space and
   * free space. Files and space are represented by a list of integers with the
   * following spec:
   * - Including the 0th index, even-index integers represent the number of
   * blocks in a file, and odd-index integers represent the number of free
   * blocks.
   */
private:
  std::vector<int> raw_disk; // use int because the file IDs will go > 10
  xt::xarray<int> vec_disk;

  // List of files built during the original scan.
  std::vector<File> files;
  // Heaps which store the free space in sorted (smallest index first)
  // order.
  std::vector<_free_heap_t> free_heaps;

public:
  DiskMap(std::ifstream &file)
      : free_heaps(11) { // initialize 11 so we can access w/ indexes 1-10
    char c;
    unsigned file_id = 0;
    bool is_file = true;
    unsigned disk_block_index = 0;
    while (file >> c) {
      unsigned num_blocks = to_num(c);
      while (to_num(c--) > 0) {
        if (is_file) {
          raw_disk.push_back(file_id);
        } else {
          raw_disk.push_back(FREE_INT);
        }
      }
      if (num_blocks) {
        if (is_file) {
          files.push_back(File(disk_block_index, num_blocks, file_id));
          // std::cout << "Adding file block at index " << disk_block_index
          //           << ", with size " << num_blocks << " to heap with file ID
          //           "
          //           << file_id << std::endl;
          file_id++;
        } else {
          FreeSpace s(disk_block_index, num_blocks);
          free_heaps[num_blocks].push(s);
          // std::cout << "Adding free block at index " << disk_block_index
          //           << ", with size " << num_blocks
          //           << " to heap with num_blocks" << num_blocks << std::endl;
        }
      }
      is_file = !is_file;
      disk_block_index += num_blocks;
    }

    // Adapt the utility parsed disk var into the vectorized format.
    vec_disk = xt::adapt(raw_disk);
  }

  void compact_part_two() {
    // Part two only allows moving contiguous files. Therefore, we now compact
    // the disk with this algorithm:
    //
    // - For each file ID _in descending order_, find the leftmost chunk of free
    // space that is >= the size of the file.
    // - Set the final index of the blocks at that location to be the location
    // of the current file, and the final index of the original file location to
    // be the leftmost blocks in the found location.
    //
    // - Instead of using the previous algorithm with two disk pointers, we now
    // make use of the File and FreeBlock data structures.
    xt::xarray<size_t> index_swaps = xt::arange<size_t>(0, vec_disk.size(), 1);

    for (std::vector<File>::reverse_iterator it = files.rbegin();
         it != files.rend(); it++) {
      // std::cout << "Compacting file: " << it->get_file_id() << std::endl;
      // Take this file and get all the options of the free blocks
      // that would fit it, by accessing the top() of each heap
      // whose size corresponds to the file's number of blocks OR
      // GREATER. Among these options, choose the one whose index
      // is the furthest left.
      std::vector<FreeSpace> compact_options;
      for (size_t min_size = it->get_size(); min_size <= MAX_FILE_SIZE;
           min_size++) {
        if (!free_heaps[min_size].empty())
          compact_options.push_back(free_heaps[min_size].top());
      }
      if (compact_options.size()) {
        // Only can do something if there is at least one destination block that
        // can take this file.
        std::sort(compact_options.begin(), compact_options.end());
        FreeSpace dest = compact_options.front();
        // std::cout << "Picked free space block at index: " << dest.get_index()
        //           << ", with size " << dest.get_size() << std::endl;

        // Starting from the beginning of the free block in 'dest' and the file
        // block in (*it), set the list of index swaps accordingly.
        assert(it->get_size() <= dest.get_size());
        for (size_t s = 0; s < it->get_size(); s++) {
          index_swaps[dest.get_index() + s] = it->get_index() + s;
          index_swaps[it->get_index() + s] = dest.get_index() + s;
        }
        size_t blocks_remaining = dest.get_size() - it->get_size();

        // Remove the old free space from its respective heap, and then
        // create a new free space with the number of blocks remaining.
        free_heaps[dest.get_size()].pop(); // guaranteed to remove the same
                                           // element as above when we did top()
        if (blocks_remaining) {
          size_t starting_index = dest.get_index() + it->get_size();
          FreeSpace remaining_space(starting_index, blocks_remaining);
          // std::cout << "Pushing a new free space block starting from index "
          //           << starting_index << " with size " << blocks_remaining
          //           << std::endl;
          free_heaps[blocks_remaining].push(remaining_space);
        }
      } else {
        // std::cout << "No free space remaining, can't compact file "
        //           << it->get_file_id() << std::endl;
      }
    }

    // All files are copmacted, calculate the checksum
    auto compacted = xt::index_view(vec_disk, index_swaps);
    // Calculate the disk checksum.
    unsigned long idx = 0;
    unsigned long long checksum = 0;
    for (auto &c : compacted) {
      if (c != FREE_INT) {
        checksum += (c * idx);
      }
      idx++;
    }
    std::cout << "Part two checksum: " << checksum << std::endl;
  }

  void compact_part_one() {
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
      if (vec_disk[free_idx] != FREE_INT) {
        index_swaps[free_idx] = free_idx;
        free_idx += 1;
      } else {
        while (vec_disk[file_idx] == FREE_INT && (free_idx < file_idx)) {
          index_swaps[file_idx] = file_idx;
          file_idx -= 1;
        }
        if (free_idx >= file_idx)
          break;
        // Getting here means that file_idx points to a non-free space block
        // that we can swap into the place pointed to by 'free_idx'.
        index_swaps[free_idx] = file_idx;
        index_swaps[file_idx] = free_idx;
        free_idx += 1;
        file_idx -= 1;
      }
    }
    auto compacted = xt::index_view(vec_disk, index_swaps);
    // Calculate the disk checksum.
    unsigned long idx = 0;
    unsigned long long checksum = 0;
    for (auto &c : compacted) {
      if (c != FREE_INT) {
        checksum += (c * idx);
      }
      idx++;
    }
    std::cout << "Part one checksum: " << checksum << std::endl;
  }
};

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
  disk.compact_part_one();
  disk.compact_part_two();
  return 0;
}
