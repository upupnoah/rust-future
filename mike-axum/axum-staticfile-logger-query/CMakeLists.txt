# CMakeLists.txt

cmake_minimum_required(VERSION 3.12)

project(axum_staticfile_logger_query)

add_custom_target(run COMMAND cargo run WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})
add_custom_target(trace COMMAND RUST_LOG=trace cargo run WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})
add_custom_target(clean_rust COMMAND cargo clean WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})

# Add other CMake configurations and targets as needed
