cmake_minimum_required(VERSION 2.8.12)

project(original C)

set(REVISION f221d8199d8f4bacdaf539b1a5138f2ce0dd578e)

file(DOWNLOAD
    https://raw.githubusercontent.com/wangyi-fudan/wyhash/${REVISION}/wyhash.h
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash.h)

add_library(original
    ${CMAKE_CURRENT_SOURCE_DIR}/lib.c
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash.h)

target_include_directories(original PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR})

install(TARGETS original DESTINATION .)
