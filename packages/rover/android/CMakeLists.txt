cmake_minimum_required(VERSION 3.18.1)
project(roverjsi)

set(CMAKE_CXX_STANDARD 17)

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/../cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main/cpp
)

# Location of prebuilt librelay_rover.so compiled by cargo
if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs/${ANDROID_ABI}/librelay_rover.so")
    add_library(relay_rover SHARED IMPORTED)
    set_target_properties(relay_rover PROPERTIES
        IMPORTED_LOCATION "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs/${ANDROID_ABI}/librelay_rover.so"
    )
endif()

add_library(roverjsi SHARED
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main/cpp/rover-jni.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/../cpp/RoverJsi.cpp
)

find_library(LOG_LIB log)
find_library(ANDROID_LIB android)

target_link_libraries(roverjsi
    PRIVATE
    relay_rover
    ${LOG_LIB}
    ${ANDROID_LIB}
)
