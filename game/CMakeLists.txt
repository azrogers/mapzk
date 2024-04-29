add_executable(mapzk-game "")
configure_mzk_component(mapzk-game)

mzk_glob_files(
  MAPZK_GAME_SOURCES
  ${CMAKE_CURRENT_LIST_DIR}/src/**/**/**/*.cpp
  ${CMAKE_CURRENT_LIST_DIR}/src/**/**/*.cpp
  ${CMAKE_CURRENT_LIST_DIR}/src/**/*.cpp
  ${CMAKE_CURRENT_LIST_DIR}/src/*.cpp
  ${CMAKE_CURRENT_LIST_DIR}/src/resources.rc)

mzk_glob_files(
  MAPZK_GAME_HEADERS 
  ${CMAKE_CURRENT_LIST_DIR}/src/**/**/**/*.h
  ${CMAKE_CURRENT_LIST_DIR}/src/**/**/*.h 
  ${CMAKE_CURRENT_LIST_DIR}/src/**/*.h
  ${CMAKE_CURRENT_LIST_DIR}/src/*.h)

target_sources(
  mapzk-game
  PRIVATE ${MAPZK_GAME_SOURCES}
  PUBLIC ${MAPZK_GAME_HEADERS})

find_path(PNGPP_INCLUDE_DIRS "png++/color.hpp")

target_include_directories(mapzk-game PRIVATE ${PNGPP_INCLUDE_DIRS} PUBLIC ${CMAKE_CURRENT_LIST_DIR}/src)

find_package(vsg CONFIG REQUIRED)
find_package(vsgXchange CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(cppfs CONFIG REQUIRED)

target_link_libraries(
  mapzk-game
  PRIVATE 
	spdlog::spdlog
	vsg::vsg 
	vsgXchange::vsgXchange
	cppfs::cppfs
	clauser)
