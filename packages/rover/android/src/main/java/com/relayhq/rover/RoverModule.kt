package com.relayhq.rover

import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition

/**
 * Expo Module implementation for Relay Rover on Android.
 * Integrates Rust core through UniFFI bindings and exposes high-performance JSI methods.
 */
class RoverModule : Module() {
  override fun definition() = ModuleDefinition {
    Name("RelayRover")

    Function("classify") { subject: String, body: String ->
      RoverNativeBridge.classify(subject, body)
    }

    Function("generatePreview") { subject: String, body: String ->
      RoverNativeBridge.generatePreview(subject, body)
    }

    Function("processEmail") { subject: String, body: String ->
      RoverNativeBridge.processEmail(subject, body)
    }

    Function("processEmailJson") { emailJson: String ->
      RoverNativeBridge.processEmailJson(emailJson)
    }

    Function("processBatchJson") { batchJson: String ->
      RoverNativeBridge.processBatchJson(batchJson)
    }

    Function("htmlToText") { html: String ->
      RoverNativeBridge.htmlToText(html)
    }

    Function("cleanEmailBody") { bodyOrHtml: String ->
      RoverNativeBridge.cleanEmailBody(bodyOrHtml)
    }

    AsyncFunction("processEmailAsync") { emailJson: String ->
      RoverNativeBridge.processEmailJson(emailJson)
    }

    AsyncFunction("processBatchAsync") { batchJson: String ->
      RoverNativeBridge.processBatchJson(batchJson)
    }
  }
}
