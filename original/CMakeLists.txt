cmake_minimum_required(VERSION 2.8.8)

project(wyhash-generator CXX)

file(DOWNLOAD
    https://github.com/wangyi-fudan/wyhash/raw/master/wyhash.h
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash.h)

add_executable(wyhash-gen
    ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash.h)

target_include_directories(wyhash-gen PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR})
