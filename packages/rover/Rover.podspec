require 'json'

package = JSON.parse(File.read(File.join(__dir__, 'package.json')))

Pod::Spec.new do |s|
  s.name         = "Rover"
  s.version      = package['version']
  s.summary      = package['description']
  s.homepage     = "https://blogs.relaymail.online/blog/rover"
  s.license      = package['license']
  s.authors      = package['author']
  s.platforms    = { :ios => "13.4" }
  s.source       = { :git => "https://github.com/RelayHQX/rover.git", :tag => "v#{s.version}" }

  s.source_files = "ios/**/*.{h,m,mm,swift}", "cpp/**/*.{h,cpp}"
  s.public_header_files = "ios/**/*.h", "cpp/**/*.h"

  s.pod_target_xcconfig = {
    "CLANG_CXX_LANGUAGE_STANDARD" => "c++17",
    "SWIFT_INCLUDE_PATHS" => "$(PODS_TARGET_SRCROOT)/ios",
    "OTHER_SWIFT_FLAGS" => "-import-objc-header $(PODS_TARGET_SRCROOT)/ios/Rover-Bridging-Header.h"
  }

  s.dependency "React-Core"
  s.dependency "React-callinvoker"

  if defined?(install_modules_dependencies)
    install_modules_dependencies(s)
  else
    s.dependency "ExpoModulesCore" rescue nil
  end
end
