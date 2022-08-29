project(Lib)

add_library(${PROJECT_NAME} src/InetAddr.cc src/lib.cc src/Socket.cc)
add_library(unix_rs::lib ALIAS ${PROJECT_NAME})

target_include_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/include)