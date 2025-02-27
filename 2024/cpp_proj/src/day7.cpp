
#include <cstdint>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

class Equation {
protected:
  int64_t final_value;
  std::vector<int64_t> components;

public:
  Equation(std::string &line) {
    // Take a line of text and parse it into the final_value and components
    // properties, following the format:
    //    final_value : component1\scomponent2\scomponent3\s...\n
    size_t delimiter = line.find(":");
    final_value = std::stoll(line.substr(0, delimiter));
    std::string components_str = line.substr(delimiter + 1);
    std::istringstream iss(components_str);
    while (iss.good()) {
      std::string component_str;
      iss >> component_str;
      components.push_back(std::stoll(component_str));
    }
  }

  Equation(int64_t val, std::vector<int64_t> comps) : final_value(val) {
    components = std::vector<int64_t>(comps);
  }

  friend std::ostream &operator<<(std::ostream &os, const Equation &eq);

  const uint64_t get_final_value() { return final_value; }

  const bool is_satisfiable() {
    // Check if the equation is satisfiable recursively.
    // The base case is when the equation has only two components, for which we
    // can simply check if either addition or multiplication of the two
    // components equals the final values.
    // Recursively check a larger equation by removing the last component
    // (because equations are always left-associative) and checking if the
    // remaining equation is satisfiable.
    if (components.size() == 2) {
      return (components[0] + components[1] == final_value) ||
             (components[0] * components[1] == final_value);
    }
    std::vector<int64_t> remaining_components(components.begin(),
                                              components.end() - 1);
    int64_t dropped_component = components.back();
    components.pop_back();

    // Don't need to check the subtractive branch if the smaller equation's
    // final_value is negative or the division branch if the smaller equation's
    // final_value is not divisible by the dropped component
    bool satisfiable_subtractive = false;
    bool satisfiable_division = false;
    if (this->final_value - dropped_component > 0) {
      satisfiable_subtractive =
          Equation(this->final_value - dropped_component, remaining_components)
              .is_satisfiable();
    }
    if (this->final_value % dropped_component == 0) {
      satisfiable_division =
          Equation(this->final_value / dropped_component, remaining_components)
              .is_satisfiable();
    }
    return satisfiable_subtractive || satisfiable_division;
  }
};

std::ostream &operator<<(std::ostream &os, const Equation &eq) {
  os << eq.final_value << ": ";
  for (int64_t component : eq.components) {
    os << " " << component;
  }
  os << std::endl;
  return os;
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input_file>" << std::endl;
  }

  // Read the input file and parse it into a vector of Equation objects.
  // Each line of the input file represents an Equation object.
  std::cout << "Parsing the input..." << std::endl;
  char *input_file = argv[1];
  std::vector<Equation> equations;
  std::fstream finput;
  finput.open(input_file, std::fstream::in);
  if (finput.good()) {
    std::string line;
    while (std::getline(finput, line)) {
      equations.push_back(Equation(line));
    }
  }

  std::cout << "Spinning up the equation processors..." << std::endl;
  int64_t sum_of_satisfied_equations = 0;
  int64_t trip_count = 0;
  for (Equation &equation : equations) {
    // Print a progress message every 128 equations
    if ((trip_count > 0) && (trip_count & ((1 << 7) - 1)) == 0) {
      std::cout << ".";
    }
    // Check if the equation is satisfied
    // std::cout << "Checking equation: " << equation << std::endl;
    if (equation.is_satisfiable()) {
      sum_of_satisfied_equations += equation.get_final_value();
    }
    ++trip_count;
  }
  std::cout << std::endl
            << "Done! Total of satisfiable equations is "
            << sum_of_satisfied_equations << std::endl;
  return 0;
}
