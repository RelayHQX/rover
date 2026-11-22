#include <jni.h>
#include <jsi/jsi.h>
#include "RoverJsi.h"

extern "C" JNIEXPORT void JNICALL
Java_com_relayhq_rover_RoverJsiModule_nativeInstall(
    JNIEnv* env,
    jobject /* this */,
    jlong jsiRuntimePointer
) {
    if (jsiRuntimePointer != 0) {
        auto* runtime = reinterpret_cast<facebook::jsi::Runtime*>(jsiRuntimePointer);
        relay_rover::RoverHostObject::install(*runtime);
    }
}
