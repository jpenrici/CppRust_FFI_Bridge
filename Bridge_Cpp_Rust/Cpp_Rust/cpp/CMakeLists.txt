cmake_minimum_required(VERSION 3.25 FATAL_ERROR)

project(CppRust_Training LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

set(LIBRARY_NAME smartex)

set(INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)
set(SRC_DIR ${CMAKE_CURRENT_SOURCE_DIR}/src)
set(LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib)
set(BIN_DIR ${CMAKE_CURRENT_SOURCE_DIR}/bin)

add_library(${LIBRARY_NAME} SHARED
    ${SRC_DIR}/example.cpp
)

target_include_directories(${LIBRARY_NAME} PUBLIC ${INCLUDE_DIR})

set_target_properties(${LIBRARY_NAME} PROPERTIES
    OUTPUT_NAME ${LIBRARY_NAME}
    LIBRARY_OUTPUT_DIRECTORY ${LIB_DIR}
)

set(TEST smart_test)

add_executable(${TEST}
    src/main.cpp
)

target_include_directories(${TEST} PRIVATE ${INCLUDE_DIR})
target_link_libraries(${TEST} PRIVATE ${LIBRARY_NAME})

set_target_properties(${TEST} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY  ${BIN_DIR})
