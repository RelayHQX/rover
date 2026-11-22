package com.relayhq.rover

/**
 * JNI wrapper to install the RoverHostObject directly into the Hermes / React Native JSI runtime.
 */
object RoverJsiModule {
    init {
        try {
            System.loadLibrary("roverjsi")
        } catch (_: UnsatisfiedLinkError) {
            // Handled when embedded directly
        }
    }

    external fun nativeInstall(jsiRuntimePointer: Long)
}
