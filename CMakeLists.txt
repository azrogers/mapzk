# CMake file adapted from
# https://github.com/CesiumGS/cesium-native/blob/main/CMakeLists.txt, used under
# Apache license

cmake_minimum_required(VERSION 3.15)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake")
set(CMAKE_CXX_STANDARD 20)

project(
  mapzk
  VERSION 0.1.0
  LANGUAGES CXX C)

set(MZK_DEBUG_POSTFIX "d")
set(MZK_RELEASE_POSTFIX "")

set(CMAKE_DEBUG_POSTFIX ${MZK_DEBUG_POSTFIX})
set(CMAKE_RELEASE_POSTFIX ${MZK_RELEASE_POSTFIX})
set(CMAKE_MINSIZEREL_POSTFIX ${MZK_RELEASE_POSTFIX})
set(CMAKE_RELWITHDEBINFO_POSTFIX ${MZK_RELEASE_POSTFIX})

function(mzk_glob_files out_var_name regexes)
  set(files "")
  foreach(arg ${ARGV})
    list(APPEND regexes_only "${arg}")
  endforeach()
  list(POP_FRONT regexes_only)
  if(GLOB_USE_CONFIGURE_DEPENDS)
    file(GLOB_RECURSE files CONFIGURE_DEPENDS ${regexes_only})
  else()
    file(GLOB files ${regexes_only})
  endif()
  set(${ARGV0}
      "${files}"
      PARENT_SCOPE)
endfunction()

function(configure_mzk_component targetName)
  set_target_properties(
    ${targetName}
    PROPERTIES CXX_STANDARD 20
               CXX_STANDARD_REQUIRED YES
               CXX_EXTENSIONS NO)
endfunction()

add_subdirectory(game)

# clang-format target

include(${CMAKE_CURRENT_LIST_DIR}/cmake/CPM.cmake)
cpmaddpackage(
  NAME
  Format.cmake
  VERSION
  1.7.3
  GITHUB_REPOSITORY
  TheLartians/Format.cmake
  OPTIONS
  # set to yes skip cmake formatting
  "FORMAT_SKIP_CMAKE NO"
  # set to yes skip clang formatting
  "FORMAT_SKIP_CLANG NO"
  # path to exclude (optional, supports regular expressions)
  "CMAKE_FORMAT_EXCLUDE cmake/CPM.cmake")
