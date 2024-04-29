#include "MapDefinition.h"

#include <spdlog/spdlog.h>
#include <fmt/format.h>
#include "clauser/ClParseStatus.h"
#include "clauser/ClReader.h"
#include "clauser/ClClassMapping.h"

MapDefinition::MapDefinition() :
    wrapX(false)
{
}

bool MapDefinition::Create(const std::string& mapFileText, MapDefinition& outMapDef)
{
    ClParseState state(mapFileText);
    ClReader reader(state);

    ClClassMapping mapping(state);
    mapping.AddMapping("provinces", &outMapDef.provincesFile);
    mapping.AddMapping("topology", &outMapDef.topologyFile);
    mapping.AddMapping("rivers", &outMapDef.riversFile);
    mapping.AddMapping("adjacencies", &outMapDef.adjacenciesFile);
    mapping.AddMapping("wrap_x", &outMapDef.wrapX);
    mapping.AddMapping("sea_starts", &outMapDef.seaStarts);
    mapping.AddMapping("lakes", &outMapDef.lakes);

    ClParseStatus status;
    if (!mapping.ReadObject(reader, state, status)) {
        status.GetError().Log();
        return false;
    }

    return true;
}
