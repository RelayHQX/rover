import ExpoModulesCore
import Foundation

/**
 * Expo Module implementation for Relay Rover on iOS.
 * Integrates Rust core through UniFFI Swift bindings and exposes high-performance JSI methods.
 */
public class RoverModule: Module {
  public func definition() -> ModuleDefinition {
    Name("RelayRover")

    Function("classify") { (subject: String, body: String) -> String in
      return classify(subject: subject, body: body)
    }

    Function("generatePreview") { (subject: String, body: String) -> String in
      return generatePreview(subject: subject, body: body)
    }

    Function("processEmail") { (subject: String, body: String) -> String in
      return processEmail(subject: subject, body: body)
    }

    Function("processEmailJson") { (emailJson: String) -> String in
      return processEmailJson(emailJson: emailJson)
    }

    Function("processBatchJson") { (batchJson: String) -> String in
      return processBatchJson(batchJson: batchJson)
    }

    Function("htmlToText") { (html: String) -> String in
      return htmlToText(html: html)
    }

    Function("cleanEmailBody") { (bodyOrHtml: String) -> String in
      return cleanEmailBody(bodyOrHtml: bodyOrHtml)
    }

    AsyncFunction("processEmailAsync") { (emailJson: String, promise: Promise) in
      DispatchQueue.global(qos: .userInitiated).async {
        let result = processEmailJson(emailJson: emailJson)
        promise.resolve(result)
      }
    }

    AsyncFunction("processBatchAsync") { (batchJson: String, promise: Promise) in
      DispatchQueue.global(qos: .userInitiated).async {
        let result = processBatchJson(batchJson: batchJson)
        promise.resolve(result)
      }
    }
  }
}
