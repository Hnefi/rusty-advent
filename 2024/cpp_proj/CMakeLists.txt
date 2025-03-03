cmake_minimum_required(VERSION 3.15)
project(
  advent_cpp
  DESCRIPTION "Advent 2024 in C++"
  LANGUAGES CXX)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Werror")
set(CMAKE_CXX_FLAGS_DEBUG
    "${CMAKE_CXX_FLAGS_DEBUG} -O0 -g -fsanitize=address -fno-omit-frame-pointer"
)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(xtensor REQUIRED)
find_package(xtl REQUIRED)

add_executable(day7 src/day7.cpp)
add_executable(day8 src/day8.cpp)
add_executable(day9 src/day9.cpp)

target_include_directories(day8 PUBLIC ${xtensor_INCLUDE_DIRS})
target_link_libraries(day8 PUBLIC xtensor)
