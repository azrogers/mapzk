#pragma once

#include <memory>

#include <vsg/all.h>

#include "CameraManager.h"

class RenderingContext {
public:
	RenderingContext();
	bool Initialize(int width, int height, const std::string& title);

	void RunLoop();

	RenderingContext(const RenderingContext&) = delete;
	void operator=(const RenderingContext&) = delete;

private:
	std::unique_ptr<CameraManager> cameraManager;
	bool isInitialized = false;

	vsg::ref_ptr<vsg::Viewer> viewer;
	vsg::ref_ptr<vsg::Options> options;
	vsg::ref_ptr<vsg::Node> rootNode;
	vsg::ref_ptr<vsg::Window> window;
};