#include <vsg/all.h>
#include <vsgXchange/all.h>
#include <spdlog/spdlog.h>

#include <initializer_list>
#include <string>

#include "RenderingContext.h"
#include "data/MapDefinition.h"
#include "util/ContentLoader.h"

#include "clauser/ClTokenizer.h"

int main(int argc, char** argv)
{
    if (argc < 2) {
        spdlog::error("Usage: mapzk <content path>");
        return 1;
    }

    ContentLoader loader({ std::string(argv[1]) });

    vsg::ref_ptr<vsg::Image> heightmap = loader.Read<vsg::Image>("game/map_data/rivers.png");
    std::optional<std::string> mapDefText = loader.ReadString("game/map_data/default.map");
    if (!mapDefText.has_value()) {
        spdlog::error("failed to load map definition");
        return 2;
    }

    MapDefinition definition;
    if (!MapDefinition::Create(mapDefText.value(), definition)) {
        spdlog::error("failed to parse map definition");
        return 3;
    }

    RenderingContext context;

    context.Initialize(800, 600, std::string("mapzk"));
    context.RunLoop();

    return 0;
}