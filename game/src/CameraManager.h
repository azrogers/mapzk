#pragma once

#include <vsg/all.h>

class CameraManager {
public:
	CameraManager(vsg::ref_ptr<vsg::Window>& window);

	void OnResize();

	vsg::ref_ptr<vsg::Camera> GetCamera() const;

	CameraManager(const CameraManager&) = delete;
	void operator=(const CameraManager&) = delete;

private:
	void UpdateProjectionMatrix();

	float nearDistance = 0.001f;
	float farDistance = 10000.0f;

	vsg::ref_ptr<vsg::Camera> camera;
	vsg::ref_ptr<vsg::Window> window;
	vsg::ref_ptr<vsg::ViewportState> viewportState;

	vsg::ref_ptr<vsg::Perspective> projection;
	vsg::ref_ptr<vsg::ViewMatrix> view;
};