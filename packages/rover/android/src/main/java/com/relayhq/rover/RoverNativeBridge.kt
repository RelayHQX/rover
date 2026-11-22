package com.relayhq.rover

import uniffi.relay_rover.*

/**
 * High-performance Kotlin bridge connecting UniFFI-generated Rust bindings
 * to Expo and React Native runtime.
 */
object RoverNativeBridge {
    init {
        try {
            System.loadLibrary("relay_rover")
        } catch (_: UnsatisfiedLinkError) {
            // In Android/JNA environments, UniFFI or system dynamic loader locates the library
        }
    }

    fun classify(subject: String, body: String): String {
        return uniffi.relay_rover.classify(subject, body)
    }

    fun generatePreview(subject: String, body: String): String {
        return uniffi.relay_rover.generatePreview(subject, body)
    }

    fun processEmail(subject: String, body: String): String {
        return uniffi.relay_rover.processEmail(subject, body)
    }

    fun processEmailJson(emailJson: String): String {
        return uniffi.relay_rover.processEmailJson(emailJson)
    }

    fun processBatchJson(batchJson: String): String {
        return uniffi.relay_rover.processBatchJson(batchJson)
    }

    fun htmlToText(html: String): String {
        return uniffi.relay_rover.htmlToText(html)
    }

    fun cleanEmailBody(bodyOrHtml: String): String {
        return uniffi.relay_rover.cleanEmailBody(bodyOrHtml)
    }
}
