project(Server)

add_executable(${PROJECT_NAME} main.cc)
target_link_libraries(${PROJECT_NAME} unix_rs::lib)