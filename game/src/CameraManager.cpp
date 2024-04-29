#include "CameraManager.h"
#include "CameraManager.h"

CameraManager::CameraManager(vsg::ref_ptr<vsg::Window>& window) :
    window(window)
{
    this->viewportState = vsg::ViewportState::create(window->extent2D());
    this->view = vsg::LookAt::create(vsg::dvec3(0, 0, 0), vsg::dvec3(0, 0, 1), vsg::dvec3(0, 1, 0));
    this->projection = vsg::Perspective::create();
    this->UpdateProjectionMatrix();

    this->camera = vsg::Camera::create(this->projection, this->view, this->viewportState);
}

void CameraManager::OnResize()
{
    this->viewportState->set(0, 0, window->extent2D().width, window->extent2D().height);
    this->UpdateProjectionMatrix();
}

vsg::ref_ptr<vsg::Camera> CameraManager::GetCamera() const { return this->camera; }

void CameraManager::UpdateProjectionMatrix()
{
    this->projection->changeExtent(this->window->extent2D(), this->window->extent2D());
    this->projection->fieldOfViewY = 90.0;
    this->projection->nearDistance = this->nearDistance;
    this->projection->farDistance = this->farDistance;
}
