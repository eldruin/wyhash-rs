cmake_minimum_required(VERSION 2.8.12)

project(original C)

set(REVISION_V1 f221d8199d8f4bacdaf539b1a5138f2ce0dd578e)
set(REVISION_FINAL3 896d7c57f68d96938f718c0695c44866d14b48d6)

file(DOWNLOAD
    https://raw.githubusercontent.com/wangyi-fudan/wyhash/${REVISION_V1}/wyhash.h
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash_v1.h)

file(DOWNLOAD
    https://raw.githubusercontent.com/wangyi-fudan/wyhash/${REVISION_FINAL3}/wyhash.h
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash_final3.h)

include_directories(${CMAKE_CURRENT_BINARY_DIR})

add_library(original_v1
    ${CMAKE_CURRENT_SOURCE_DIR}/lib_v1.c
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash_v1.h)

add_library(original_final3
    ${CMAKE_CURRENT_SOURCE_DIR}/lib_final3.c
    ${CMAKE_CURRENT_BINARY_DIR}/wyhash_final3.h)

install(TARGETS original_v1 original_final3 DESTINATION .)

