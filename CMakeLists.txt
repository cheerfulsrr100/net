cmake_minimum_required(VERSION 3.16)
project(unix_rs)

set(CMAKE_CXX_STANDARD 11)

add_executable(unix_rs src/main.cpp src/InetAddr.cc src/InetAddr.h src/Socket.cc src/Socket.h)