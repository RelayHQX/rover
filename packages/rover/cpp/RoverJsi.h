#pragma once

#include <jsi/jsi.h>
#include <memory>
#include <vector>

namespace relay_rover {

class RoverHostObject : public facebook::jsi::HostObject {
public:
    RoverHostObject();
    ~RoverHostObject() override = default;

    facebook::jsi::Value get(facebook::jsi::Runtime& runtime, const facebook::jsi::PropNameID& name) override;
    std::vector<facebook::jsi::PropNameID> getPropertyNames(facebook::jsi::Runtime& runtime) override;

    static void install(facebook::jsi::Runtime& runtime);
};

} // namespace relay_rover
