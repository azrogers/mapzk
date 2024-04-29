#pragma once

#include <vsg/all.h>

#include "../util/ContentLoader.h"

class TerrainLoader {
	static vsg::ref_ptr<vsg::Node> Load(const std::string& mapDefPath);
};