#include "RenderingContext.h"

#include <spdlog/spdlog.h>

RenderingContext::RenderingContext()
{
	this->rootNode = vsg::Group::create();
}

bool RenderingContext::Initialize(int width, int height, const std::string& title)
{
	this->viewer = vsg::Viewer::create();

	vsg::ref_ptr<vsg::WindowTraits> traits = vsg::WindowTraits::create(width, height, title);
	this->window = vsg::Window::create(traits);

	this->cameraManager = std::make_unique<CameraManager>(this->window);

	this->viewer->addEventHandler(vsg::CloseHandler::create(viewer));

	vsg::ref_ptr<vsg::CommandGraph> commandGraph =
		vsg::createCommandGraphForView(this->window, this->cameraManager->GetCamera(), this->rootNode);
	this->viewer->assignRecordAndSubmitTaskAndPresentation({ commandGraph });

	this->viewer->compile();

	this->isInitialized = true;
	return true;
}

void RenderingContext::RunLoop()
{
	if (!this->isInitialized) {
		spdlog::error("Tried to call RunLoop before Initialize!");
		return;
	}

	while (this->viewer->advanceToNextFrame()) {
		this->viewer->handleEvents();
		this->viewer->update();
		this->viewer->recordAndSubmit();
		this->viewer->present();
	}
}
