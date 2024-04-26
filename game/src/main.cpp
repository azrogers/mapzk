#include <vsg/all.h>
#include <vsgXchange/all.h>

#include "RenderingContext.h"

int main(int argc, char** argv) {
	RenderingContext context;

	context.Initialize(800, 600, std::string("mapzk"));
	context.RunLoop();

	return 0;
}