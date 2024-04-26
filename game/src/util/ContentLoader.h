#pragma once

#include <vsg/all.h>
#include <spdlog/spdlog.h>

#include <map>
#include <string>
#include <vector>

/**
 * A virtual filesystem representing a combination of files from many sources as a single filesystem.
 */
class ContentLoader {
public:
	ContentLoader(const std::vector<std::string>& paths);

	ContentLoader(const ContentLoader&) = delete;
	void operator=(const ContentLoader&) = delete;

	template <typename T> vsg::ref_ptr<T> Read(const std::string& filename) const;

private:
	std::optional<std::string> FindFilePath(const std::string& filename) const;

	std::vector<std::string> searchPaths;
	vsg::ref_ptr<vsg::Options> options;
};

template <typename T> inline vsg::ref_ptr<T> ContentLoader::Read(const std::string& filename) const
{
	const std::optional<std::string> path = this->FindFilePath(filename);
	if (!path.has_value()) {
		spdlog::error("failed to find {} at any search path", filename);
		return T::create();
	}

	return vsg::read_cast<T>(path.value(), this->options);
}
