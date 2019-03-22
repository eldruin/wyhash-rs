cmake_minimum_required(VERSION 2.8.8)

project(wyhash-generator CXX)

set(REVISION f221d8199d8f4bacdaf539b1a5138f2ce0dd578e)

file(DOWNLOAD
    https://raw.githubusercontent.com/wangyi-fudan/wyhash/${REVISION}/wyhash.h
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash.h)

add_executable(wyhash-gen
    ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash.h)

target_include_directories(wyhash-gen PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR})
