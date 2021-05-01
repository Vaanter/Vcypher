cmake_minimum_required(VERSION 3.16)
project(VCypher C CXX)

##
### Test definitions ###
##

configure_file(CMakeLists.txt.in
        googletest-download/CMakeLists.txt)
execute_process(COMMAND ${CMAKE_COMMAND} -G "${CMAKE_GENERATOR}" .
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/googletest-download )
execute_process(COMMAND ${CMAKE_COMMAND} --build .
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/googletest-download )

add_subdirectory(${CMAKE_BINARY_DIR}/googletest-src
        ${CMAKE_BINARY_DIR}/googletest-build)

enable_testing()
add_subdirectory(tests)

include_directories("${PROJECT_SOURCE_DIR}/tests")

file(GLOB sources "${PROJECT_SOURCE_DIR}/*.c" "${PROJECT_SOURCE_DIR}/*.h")

add_executable(vcypher ${sources})
target_link_libraries(vcypher m)
target_link_libraries(vcypher gmp)