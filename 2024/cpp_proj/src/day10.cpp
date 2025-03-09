#include <deque>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <vector>

const char ZERO = '0';

class HikingMapGraph {
  class Trail {
  public:
    size_t location;
    unsigned elevation; // a number from 0-9 signifying elevation.
    std::set<size_t> summits_reachable; // the locations of all reachable
                                        // summits from this trail

    Trail(size_t loc, char anElevation)
        : location(loc), elevation((size_t)anElevation), summits_reachable() {}
    bool is_trail_head() const { return elevation == 0; }
    bool is_summit() const { return elevation == 9; }
    size_t get_location() const { return location; }
  };

private:
  std::vector<Trail> trails;
  std::size_t line_length;

  bool is_target_index_on_same_line(const size_t target_index,
                                    const size_t base_index) const {
    return ((base_index / line_length) == (target_index / line_length));
  }

  std::vector<Trail *> get_trails_with_increasing_height(Trail *current_trail) {
    std::vector<Trail *> next_trails;
    int32_t current_location = current_trail->get_location();
    // std::cout << "Looking at current_location " << current_location
    //           << std::endl;

    // Possible location #1: up vertically on the 2D map.
    if ((current_location - (int32_t)line_length) >= 0) {
      // std::cout << "Adding location " << current_location - line_length
      //           << std::endl;
      next_trails.push_back(&trails.at(current_location - line_length));
    }

    // Possible location #2: down vertically on the 2D map.
    if ((current_location + line_length) < trails.size()) {
      // std::cout << "Adding location " << current_location + line_length
      //           << std::endl;
      next_trails.push_back(&trails.at(current_location + line_length));
    }

    // Possible location #3: index + 1
    //  - it has to be in the array, but also on the same line, so we don't
    //  illegally consider next locations that wrap around the 2D array
    if (is_target_index_on_same_line(current_location + 1, current_location) &&
        (current_location + 1) < (int32_t)trails.size()) {
      // std::cout << "Adding location " << current_location + 1 << std::endl;
      next_trails.push_back(&trails.at(current_location + 1));
    }

    // Possible location #4: index - 1
    //  - it has to be in the array, but also on the same line (same as above)
    if (is_target_index_on_same_line(current_location - 1, current_location) &&
        (current_location - 1) >= 0) {
      // std::cout << "Adding location " << current_location - 1 << std::endl;
      next_trails.push_back(&trails.at(current_location - 1));
    }

    return next_trails;
  }

  void find_reachable_summits_helper(Trail &trail_head) {
    // Do a DFS for the trailhead provided. Implement by popping the
    // current node off the queue and then enqueuing all trails with increasing
    // height on the front, so they will be depth-first searched immediately.
    std::deque<Trail *> trail_queue;
    trail_queue.push_back(&trail_head);

    while (!trail_queue.empty()) {
      Trail *current = trail_queue.front();
      trail_queue.pop_front();
      for (Trail *trail : get_trails_with_increasing_height(current)) {
        if (trail->elevation == (current->elevation + 1)) {
          // Check the trail has increasing height compared to the current
          // location
          if (trail->is_summit()) {
            /*
            std::cout << "Reached summit at location: " << trail->get_location()
                      << " from trail head at location: "
                      << trail_head.get_location() << std::endl;
            */
            trail_head.summits_reachable.insert(trail->get_location());
          } else {
            // Only traverse the next location if it is not a summit.
            trail_queue.push_front(trail);
          }
        }
      }
    }
  }

public:
  HikingMapGraph(std::ifstream &istream) : trails() {
    // Read the stream line by line and push back a new Trail for each location.
    std::string line;
    size_t loc = 0;
    while (std::getline(istream, line)) {
      line_length = line.size();
      for (char &c : line) {
        trails.push_back(Trail(loc++, c - ZERO));
      }
    }
  }

  unsigned sum_scores() const {
    unsigned sum = 0;
    for (const Trail &trail : trails) {
      if (trail.is_trail_head()) {
        sum += trail.summits_reachable.size();
      }
    }
    return sum;
  }

  void find_reachable_summits() {
    // For each starting trail head, calculate all the summits that are
    // reachable with the following constraints:
    // 1. A summit has elevation == 9.
    // 2. A valid hiking path starts from a trailhead, going to a summit with an
    // increase of one elevation point per step.
    // Note: summits are uniquely reachable, it does not matter how many paths
    // can lead from one starting location to a summit.
    for (Trail &trail : trails) {
      if (trail.is_trail_head()) {
        find_reachable_summits_helper(trail);
      }
    }
  }
};

int main(int argc, char *argv[]) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input_file>" << std::endl;
  }

  // Read the input file line by line and create a HikingMapGraph.
  std::ifstream in;
  in.open(argv[1], std::ifstream::in);
  if (!in.is_open()) {
    std::cerr << "Failed to open file " << argv[1] << std::endl;
  }
  HikingMapGraph map(in);
  map.find_reachable_summits();
  std::cout << "Sum of all reachable summits is: " << map.sum_scores()
            << std::endl;
}
