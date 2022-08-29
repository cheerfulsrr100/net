cmake_minimum_required(VERSION 3.16)
project(unix_rs)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_subdirectory(lib)
add_subdirectory(server)
add_subdirectory(client)