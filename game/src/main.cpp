#include <vsg/all.h>
#include <vsgXchange/all.h>
#include <spdlog/spdlog.h>

#include <initializer_list>
#include <string>

#include "RenderingContext.h"
#include "util/ContentLoader.h"

int main(int argc, char** argv) {
	if (argc < 2)
	{
		spdlog::error("Usage: mapzk <content path>");
		return 1;
	}

	ContentLoader vfs({ std::string(argv[1]) });

	vsg::ref_ptr<vsg::Image> heightmap = vfs.Read<vsg::Image>("game/map_data/rivers.png");

	RenderingContext context;

	context.Initialize(800, 600, std::string("mapzk"));
	context.RunLoop();

	return 0;
}