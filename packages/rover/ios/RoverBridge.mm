#import <React/RCTBridgeModule.h>
#import <React/RCTBridge+Private.h>
#import "RoverJsi.h"

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

@interface RelayRover : NSObject <RCTBridgeModule>
@property (nonatomic, weak) RCTBridge *bridge;
@end

@implementation RelayRover
@synthesize bridge = _bridge;

RCT_EXPORT_MODULE(RelayRover)

+ (BOOL)requiresMainQueueSetup {
    return YES;
}

- (void)setBridge:(RCTBridge *)bridge {
    _bridge = bridge;
    RCTCxxBridge *cxxBridge = (RCTCxxBridge *)bridge;
    if (cxxBridge.runtime) {
        relay_rover::RoverHostObject::install(*(facebook::jsi::Runtime *)cxxBridge.runtime);
    }
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(installJsi) {
    RCTCxxBridge *cxxBridge = (RCTCxxBridge *)self.bridge;
    if (cxxBridge.runtime) {
        relay_rover::RoverHostObject::install(*(facebook::jsi::Runtime *)cxxBridge.runtime);
        return @YES;
    }
    return @NO;
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(classify:(NSString *)subject body:(NSString *)body) {
    char *res = rover_native_classify([subject UTF8String], [body UTF8String]);
    NSString *str = [NSString stringWithUTF8String:res ? res : "General"];
    rover_native_free_string(res);
    return str;
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(generatePreview:(NSString *)subject body:(NSString *)body) {
    char *res = rover_native_generate_preview([subject UTF8String], [body UTF8String]);
    NSString *str = [NSString stringWithUTF8String:res ? res : ""];
    rover_native_free_string(res);
    return str;
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(processEmail:(NSString *)subject body:(NSString *)body) {
    char *res = rover_native_process_email([subject UTF8String], [body UTF8String]);
    NSString *str = [NSString stringWithUTF8String:res ? res : "{}"];
    rover_native_free_string(res);
    return str;
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(processEmailJson:(NSString *)emailJson) {
    char *res = rover_native_process_email_json([emailJson UTF8String]);
    NSString *str = [NSString stringWithUTF8String:res ? res : "{}"];
    rover_native_free_string(res);
    return str;
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(processBatchJson:(NSString *)batchJson) {
    char *res = rover_native_process_batch_json([batchJson UTF8String]);
    NSString *str = [NSString stringWithUTF8String:res ? res : "[]"];
    rover_native_free_string(res);
    return str;
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(htmlToText:(NSString *)html) {
    char *res = rover_native_html_to_text([html UTF8String]);
    NSString *str = [NSString stringWithUTF8String:res ? res : ""];
    rover_native_free_string(res);
    return str;
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(cleanEmailBody:(NSString *)bodyOrHtml) {
    char *res = rover_native_clean_email_body([bodyOrHtml UTF8String]);
    NSString *str = [NSString stringWithUTF8String:res ? res : ""];
    rover_native_free_string(res);
    return str;
}

RCT_EXPORT_METHOD(processEmailAsync:(NSString *)emailJson
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject) {
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        char *res = rover_native_process_email_json([emailJson UTF8String]);
        NSString *str = [NSString stringWithUTF8String:res ? res : "{}"];
        rover_native_free_string(res);
        resolve(str);
    });
}

RCT_EXPORT_METHOD(processBatchAsync:(NSString *)batchJson
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject) {
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        char *res = rover_native_process_batch_json([batchJson UTF8String]);
        NSString *str = [NSString stringWithUTF8String:res ? res : "[]"];
        rover_native_free_string(res);
        resolve(str);
    });
}

@end
