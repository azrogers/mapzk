#include "MapDefinition.h"

#include <spdlog/spdlog.h>
#include <fmt/format.h>
#include "clauser/ClReader.h"
#include "clauser/ClClassMapping.h"

MapDefinition::MapDefinition() :
	wrapX(false)
{
}

bool MapDefinition::Create(const std::string& mapFileText, MapDefinition& outMapDef)
{
	MapDefinition definition;

	ClParseState state(mapFileText);
	ClReader reader(state);

	ClClassMapping mapping(state);
	mapping.AddMapping("provinces", &definition.provincesFile);
	mapping.AddMapping("topology", &definition.topologyFile);
	mapping.AddMapping("rivers", &definition.riversFile);
	mapping.AddMapping("adjacencies", &definition.adjacenciesFile);
	mapping.AddMapping("wrap_x", &definition.wrapX);
	mapping.AddMapping("sea_starts", &definition.seaStarts);
	mapping.AddMapping("lakes", &definition.lakes);

	ClParseStatus status;
	if (!mapping.ReadObject(reader, state, status)) {
		status.GetError().Log();
		return false;
	}
}
