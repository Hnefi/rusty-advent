cmake_minimum_required(VERSION 3.15)
project(
  advent_cpp
  DESCRIPTION "Advent 2024 in C++"
  LANGUAGES CXX)

set(CMAKE_CXX_FLAGS_DEBUG
    "${CMAKE_CXX_FLAGS_DEBUG} -O1 -g -fsanitize=address -fno-omit-frame-pointer"
)
add_executable(day7 src/day7.cpp)
