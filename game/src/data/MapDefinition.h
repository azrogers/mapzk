#include <vector>
#include <string>

/**
 * Parses a map definition file, such as default.map.
 */
class MapDefinition {
public:
	static bool Create(const std::string& mapFileText, MapDefinition& outMapDef);

	MapDefinition();

private:

	std::string provincesFile;
	std::string topologyFile;
	std::string riversFile;
	std::string adjacenciesFile;

	bool wrapX;
	std::vector<std::string> seaStarts;
	std::vector<std::string> lakes;
};