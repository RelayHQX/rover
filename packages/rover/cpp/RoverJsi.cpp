#include "RoverJsi.h"
#include <string>
#include <vector>
#include <future>
#include <thread>

extern "C" {
    char* rover_native_classify(const char* subject, const char* body);
    char* rover_native_generate_preview(const char* subject, const char* body);
    char* rover_native_process_email(const char* subject, const char* body);
    char* rover_native_process_email_json(const char* email_json);
    char* rover_native_process_batch_json(const char* batch_json);
    char* rover_native_html_to_text(const char* html);
    char* rover_native_clean_email_body(const char* text);
    void rover_native_free_string(char* ptr);
}

namespace relay_rover {

RoverHostObject::RoverHostObject() = default;

void RoverHostObject::install(facebook::jsi::Runtime& runtime) {
    auto hostObject = std::make_shared<RoverHostObject>();
    auto jsObject = facebook::jsi::Object::createFromHostObject(runtime, hostObject);
    runtime.global().setProperty(runtime, "__RELAY_ROVER_NATIVE__", jsObject);
}

std::vector<facebook::jsi::PropNameID> RoverHostObject::getPropertyNames(facebook::jsi::Runtime& runtime) {
    std::vector<facebook::jsi::PropNameID> names;
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "classify"));
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "generatePreview"));
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "processEmail"));
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "processEmailJson"));
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "processBatchJson"));
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "htmlToText"));
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "cleanEmailBody"));
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "processEmailAsync"));
    names.push_back(facebook::jsi::PropNameID::forAscii(runtime, "processBatchAsync"));
    return names;
}

facebook::jsi::Value RoverHostObject::get(facebook::jsi::Runtime& runtime, const facebook::jsi::PropNameID& name) {
    std::string propName = name.utf8(runtime);

    if (propName == "classify") {
        return facebook::jsi::Function::createFromHostFunction(
            runtime,
            name,
            2,
            [](facebook::jsi::Runtime& rt, const facebook::jsi::Value& thisVal, const facebook::jsi::Value* args, size_t count) -> facebook::jsi::Value {
                std::string subject = (count > 0 && args[0].isString()) ? args[0].asString(rt).utf8(rt) : "";
                std::string body = (count > 1 && args[1].isString()) ? args[1].asString(rt).utf8(rt) : "";
                char* resPtr = rover_native_classify(subject.c_str(), body.c_str());
                std::string result = resPtr ? resPtr : "General";
                rover_native_free_string(resPtr);
                return facebook::jsi::String::createFromUtf8(rt, result);
            }
        );
    }

    if (propName == "generatePreview") {
        return facebook::jsi::Function::createFromHostFunction(
            runtime,
            name,
            2,
            [](facebook::jsi::Runtime& rt, const facebook::jsi::Value& thisVal, const facebook::jsi::Value* args, size_t count) -> facebook::jsi::Value {
                std::string subject = (count > 0 && args[0].isString()) ? args[0].asString(rt).utf8(rt) : "";
                std::string body = (count > 1 && args[1].isString()) ? args[1].asString(rt).utf8(rt) : "";
                char* resPtr = rover_native_generate_preview(subject.c_str(), body.c_str());
                std::string result = resPtr ? resPtr : "";
                rover_native_free_string(resPtr);
                return facebook::jsi::String::createFromUtf8(rt, result);
            }
        );
    }

    if (propName == "processEmail") {
        return facebook::jsi::Function::createFromHostFunction(
            runtime,
            name,
            2,
            [](facebook::jsi::Runtime& rt, const facebook::jsi::Value& thisVal, const facebook::jsi::Value* args, size_t count) -> facebook::jsi::Value {
                std::string subject = (count > 0 && args[0].isString()) ? args[0].asString(rt).utf8(rt) : "";
                std::string body = (count > 1 && args[1].isString()) ? args[1].asString(rt).utf8(rt) : "";
                char* resPtr = rover_native_process_email(subject.c_str(), body.c_str());
                std::string result = resPtr ? resPtr : "{}";
                rover_native_free_string(resPtr);
                return facebook::jsi::String::createFromUtf8(rt, result);
            }
        );
    }

    if (propName == "processEmailJson") {
        return facebook::jsi::Function::createFromHostFunction(
            runtime,
            name,
            1,
            [](facebook::jsi::Runtime& rt, const facebook::jsi::Value& thisVal, const facebook::jsi::Value* args, size_t count) -> facebook::jsi::Value {
                std::string json = (count > 0 && args[0].isString()) ? args[0].asString(rt).utf8(rt) : "{}";
                char* resPtr = rover_native_process_email_json(json.c_str());
                std::string result = resPtr ? resPtr : "{}";
                rover_native_free_string(resPtr);
                return facebook::jsi::String::createFromUtf8(rt, result);
            }
        );
    }

    if (propName == "processBatchJson") {
        return facebook::jsi::Function::createFromHostFunction(
            runtime,
            name,
            1,
            [](facebook::jsi::Runtime& rt, const facebook::jsi::Value& thisVal, const facebook::jsi::Value* args, size_t count) -> facebook::jsi::Value {
                std::string json = (count > 0 && args[0].isString()) ? args[0].asString(rt).utf8(rt) : "[]";
                char* resPtr = rover_native_process_batch_json(json.c_str());
                std::string result = resPtr ? resPtr : "[]";
                rover_native_free_string(resPtr);
                return facebook::jsi::String::createFromUtf8(rt, result);
            }
        );
    }

    if (propName == "htmlToText") {
        return facebook::jsi::Function::createFromHostFunction(
            runtime,
            name,
            1,
            [](facebook::jsi::Runtime& rt, const facebook::jsi::Value& thisVal, const facebook::jsi::Value* args, size_t count) -> facebook::jsi::Value {
                std::string html = (count > 0 && args[0].isString()) ? args[0].asString(rt).utf8(rt) : "";
                char* resPtr = rover_native_html_to_text(html.c_str());
                std::string result = resPtr ? resPtr : "";
                rover_native_free_string(resPtr);
                return facebook::jsi::String::createFromUtf8(rt, result);
            }
        );
    }

    if (propName == "cleanEmailBody") {
        return facebook::jsi::Function::createFromHostFunction(
            runtime,
            name,
            1,
            [](facebook::jsi::Runtime& rt, const facebook::jsi::Value& thisVal, const facebook::jsi::Value* args, size_t count) -> facebook::jsi::Value {
                std::string text = (count > 0 && args[0].isString()) ? args[0].asString(rt).utf8(rt) : "";
                char* resPtr = rover_native_clean_email_body(text.c_str());
                std::string result = resPtr ? resPtr : "";
                rover_native_free_string(resPtr);
                return facebook::jsi::String::createFromUtf8(rt, result);
            }
        );
    }

    return facebook::jsi::Value::undefined();
}

} // namespace relay_rover
