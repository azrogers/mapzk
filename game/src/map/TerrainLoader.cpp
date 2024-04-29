#include "TerrainLoader.h"

#include <png++/png.hpp>
#include <filesystem>

const int TILE_SIZE = 512;

vsg::ref_ptr<vsg::Node> TerrainLoader::Load(const std::string& mapDefPath)
{
	std::filesystem::path path(mapDefPath);
	std::filesystem::path dirPath(path.parent_path());

	std::filesystem::path heightmapImagePath = dirPath / "heightmap.png";

	png::image<png::gray_pixel_16> image(heightmapImagePath.string());
	const int tilesCountX = image.get_width() / TILE_SIZE;
	const int tilesCountY = image.get_height() / TILE_SIZE;


}