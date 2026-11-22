cmake_minimum_required(VERSION 3.18.1)
project(roverjsi)

set(CMAKE_CXX_STANDARD 17)

# 1. Search for ReactAndroid via Prefab (modern standard for RN >= 0.71)
find_package(ReactAndroid CONFIG)

# 2. Setup Include Directories
set(ROVER_INCLUDES
    ${CMAKE_CURRENT_SOURCE_DIR}/../cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main/cpp
)

# If REACT_NATIVE_DIR is provided by Gradle, add ReactCommon and jsi headers
if(DEFINED REACT_NATIVE_DIR)
    list(APPEND ROVER_INCLUDES
        "${REACT_NATIVE_DIR}/ReactCommon"
        "${REACT_NATIVE_DIR}/ReactCommon/jsi"
        "${REACT_NATIVE_DIR}/ReactAndroid/src/main/jni"
    )
endif()

# Fallback auto-discovery of ReactCommon if neither Prefab nor REACT_NATIVE_DIR resolved headers
find_path(REACT_COMMON_DIR "jsi/jsi.h"
    PATHS
    "${REACT_NATIVE_DIR}/ReactCommon"
    "${CMAKE_CURRENT_SOURCE_DIR}/../../../../node_modules/react-native/ReactCommon"
    "${CMAKE_CURRENT_SOURCE_DIR}/../../../../../node_modules/react-native/ReactCommon"
    "${CMAKE_CURRENT_SOURCE_DIR}/../../../node_modules/react-native/ReactCommon"
    "${CMAKE_CURRENT_SOURCE_DIR}/../../node_modules/react-native/ReactCommon"
)

if(REACT_COMMON_DIR)
    list(APPEND ROVER_INCLUDES "${REACT_COMMON_DIR}")
    list(APPEND ROVER_INCLUDES "${REACT_COMMON_DIR}/jsi")
endif()

include_directories(${ROVER_INCLUDES})

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

set(ROVER_LINK_LIBS
    ${LOG_LIB}
    ${ANDROID_LIB}
)

if(TARGET relay_rover)
    list(APPEND ROVER_LINK_LIBS relay_rover)
endif()

if(TARGET ReactAndroid::jsi)
    list(APPEND ROVER_LINK_LIBS ReactAndroid::jsi)
endif()

target_link_libraries(roverjsi
    PRIVATE
    ${ROVER_LINK_LIBS}
)
