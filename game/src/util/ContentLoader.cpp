#include "ContentLoader.h"

#include <vsgXchange/all.h>
#include <spdlog/spdlog.h>
#include <cppfs/cppfs.h>
#include <cppfs/fs.h>
#include <cppfs/FileHandle.h>
#include <cppfs/FilePath.h>
#include <cppfs/FileIterator.h>
#include <filesystem>

using namespace cppfs;

ContentLoader::ContentLoader(const std::vector<std::string>& paths)
{
	this->searchPaths = std::vector<std::string>(paths);

	this->options = vsg::Options::create();
	this->options->add(vsgXchange::all::create());
}

std::optional<std::string> ContentLoader::ReadString(const std::string& filename) const
{
	const std::optional<std::string> path = this->FindFilePath(filename);
	if (!path.has_value()) {
		spdlog::error("failed to find {} at any search path", filename);
		return std::nullopt;
	}

	FileHandle handle = fs::open(path.value());
	return handle.readFile();
}

std::optional<std::string> ContentLoader::FindFilePath(const std::string& filename) const
{
	for (const std::string& rootPath : this->searchPaths) {
		std::filesystem::path filePath = std::filesystem::path(rootPath) / filename;
		FileHandle handle = fs::open(filePath.string());
		if (handle.isFile()) {
			return filePath.string();
		}
	}

	return std::nullopt;
}
