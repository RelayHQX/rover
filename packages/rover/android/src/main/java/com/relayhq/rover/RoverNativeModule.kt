package com.relayhq.rover

import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.bridge.ReactContextBaseJavaModule
import com.facebook.react.bridge.ReactMethod
import com.facebook.react.bridge.Promise

/**
 * React Native Android Bridge module for vanilla (non-Expo) React Native applications.
 */
class RoverNativeModule(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
    override fun getName(): String = "RelayRover"

    @ReactMethod(isBlockingSynchronousMethod = true)
    fun classify(subject: String, body: String): String = RoverNativeBridge.classify(subject, body)

    @ReactMethod(isBlockingSynchronousMethod = true)
    fun generatePreview(subject: String, body: String): String = RoverNativeBridge.generatePreview(subject, body)

    @ReactMethod(isBlockingSynchronousMethod = true)
    fun processEmail(subject: String, body: String): String = RoverNativeBridge.processEmail(subject, body)

    @ReactMethod(isBlockingSynchronousMethod = true)
    fun processEmailJson(emailJson: String): String = RoverNativeBridge.processEmailJson(emailJson)

    @ReactMethod(isBlockingSynchronousMethod = true)
    fun processBatchJson(batchJson: String): String = RoverNativeBridge.processBatchJson(batchJson)

    @ReactMethod(isBlockingSynchronousMethod = true)
    fun htmlToText(html: String): String = RoverNativeBridge.htmlToText(html)

    @ReactMethod(isBlockingSynchronousMethod = true)
    fun cleanEmailBody(bodyOrHtml: String): String = RoverNativeBridge.cleanEmailBody(bodyOrHtml)

    @ReactMethod
    fun processEmailAsync(emailJson: String, promise: Promise) {
        Thread {
            try {
                val res = RoverNativeBridge.processEmailJson(emailJson)
                promise.resolve(res)
            } catch (e: Throwable) {
                promise.reject("ROVER_ERROR", e.message, e)
            }
        }.start()
    }

    @ReactMethod
    fun processBatchAsync(batchJson: String, promise: Promise) {
        Thread {
            try {
                val res = RoverNativeBridge.processBatchJson(batchJson)
                promise.resolve(res)
            } catch (e: Throwable) {
                promise.reject("ROVER_ERROR", e.message, e)
            }
        }.start()
    }

    @ReactMethod(isBlockingSynchronousMethod = true)
    fun installJsi(): Boolean {
        return try {
            val jsContext = reactApplicationContext.javaScriptContextHolder?.get() ?: 0L
            if (jsContext != 0L) {
                RoverJsiModule.nativeInstall(jsContext)
                true
            } else {
                false
            }
        } catch (_: Throwable) {
            false
        }
    }
}
