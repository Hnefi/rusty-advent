#include <cstdint>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

class LeftAssociativeOperator {
  // A left-associative operator interface class, which is then instantiated by
  // concrete operators such as Add, Multiply, and Concatenate.
protected:
  int64_t lhs;
  int64_t rhs;

public:
  LeftAssociativeOperator(int64_t lhs, int64_t rhs) : lhs(lhs), rhs(rhs) {}
  virtual ~LeftAssociativeOperator() {}
  virtual int64_t operator()() const = 0;
};

class Add : public LeftAssociativeOperator {
public:
  Add(int64_t lhs, int64_t rhs) : LeftAssociativeOperator(lhs, rhs) {}
  int64_t operator()() const {
    // std::cout << "Calling add on " << lhs << "+" << rhs << std::endl;
    return lhs + rhs;
  }
};

class Multiply : public LeftAssociativeOperator {
public:
  Multiply(int64_t lhs, int64_t rhs) : LeftAssociativeOperator(lhs, rhs) {}
  int64_t operator()() const {
    // std::cout << "Calling multiply on " << lhs << "*" << rhs << std::endl;
    return lhs * rhs;
  }
};

class Concatenate : public LeftAssociativeOperator {
public:
  Concatenate(int64_t lhs, int64_t rhs) : LeftAssociativeOperator(lhs, rhs) {}
  int64_t operator()() const {
    // std::cout << "Calling concatenate on " << lhs << "||" << rhs <<
    // std::endl;
    std::string lhs_str = std::to_string(lhs);
    std::string rhs_str = std::to_string(rhs);
    std::string concatenated = lhs_str + rhs_str;
    return std::stoll(concatenated);
  }
};

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

  uint64_t get_final_value() const { return final_value; }

  bool is_satisfiable_part_two() {
    int64_t running_value = components.at(0);
    return is_satisfiable_part_two_helper(running_value,
                                          components.begin() + 1);
  }

  bool is_satisfiable_part_one() {
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
              .is_satisfiable_part_one();
    }
    if (this->final_value % dropped_component == 0) {
      satisfiable_division =
          Equation(this->final_value / dropped_component, remaining_components)
              .is_satisfiable_part_one();
    }
    return satisfiable_subtractive || satisfiable_division;
  }

private:
  void cleanup_operators(std::vector<LeftAssociativeOperator *> &ops) {
    for (LeftAssociativeOperator *op : ops) {
      delete op;
    }
  }

  bool is_satisfiable_part_two_helper(int64_t running_value,
                                      std::vector<int64_t>::iterator it) {
    // Check if the equation is satisfiable recursively, by iterating over the
    // components vector, and building up a "running value" by first applying an
    // operator to the running value and the current component. The base case is
    // when there are no more components, in which case we check if the running
    // value is the same as the final value. Otherwise, we recursively check the
    // remaining components with the new running value.

    if (it == components.end()) {
      return running_value == final_value;
    }
    // Build operators and recursively check the remaining satisfiability with
    // each one.
    std::vector<LeftAssociativeOperator *> operators = {
        new Add(running_value, *it), new Multiply(running_value, *it),
        new Concatenate(running_value, *it)};

    bool satisfiable = false;
    for (LeftAssociativeOperator *op : operators) {
      int64_t next_running_value = (*op)();
      if (next_running_value <= final_value) {
        std::vector<int64_t>::iterator next_it(it);
        if (is_satisfiable_part_two_helper(next_running_value, ++next_it)) {
          satisfiable = true;
          break;
        }
      }
    }
    cleanup_operators(operators);
    return satisfiable;
  };
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
    // Check if the equation is satisfied
    // std::cout << "Checking equation: " << equation << std::endl;
    if (equation.is_satisfiable_part_two()) {
      sum_of_satisfied_equations += equation.get_final_value();
    }
    // Print a progress message every 128 equations
    if ((++trip_count & ((1 << 7) - 1)) == 0) {
      std::cout << "." << std::flush;
    }
  }
  std::cout << std::endl
            << "Done! Total of satisfiable equations is "
            << sum_of_satisfied_equations << std::endl;
  return 0;
}
