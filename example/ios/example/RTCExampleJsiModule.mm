#import <Foundation/Foundation.h>
#import "RCTExampleJsiModule.h"

#import <React/RCTBridge+Private.h>
#import <React/RCTBridge.h>
#import <React/RCTUtils.h>
#import <jsi/jsi.h>
#import <ReactCommon/RCTTurboModule.h>

@implementation RCTExampleJsiModule

@synthesize bridge = _bridge;

RCT_EXPORT_MODULE(ExampleJsiModule)

- (void)invalidate {
  _bridge = nil;
}

- (void)setBridge:(RCTBridge *)bridge {
  _bridge = bridge;
}

+ (BOOL)requiresMainQueueSetup {
  return YES;
}
 

void installApi(std::shared_ptr<facebook::react::CallInvoker> callInvoker,
                facebook::jsi::Runtime *runtime) {
  
  ExampleJsiModule_init((void*)runtime, (std::shared_ptr<void>) callInvoker);
}

RCT_EXPORT_BLOCKING_SYNCHRONOUS_METHOD(install) {
  RCTCxxBridge *cxxBridge = (RCTCxxBridge *)_bridge;
  if (cxxBridge.runtime != nullptr) {
    auto callInvoker = cxxBridge.jsCallInvoker;
    facebook::jsi::Runtime *jsRuntime = (facebook::jsi::Runtime *)cxxBridge.runtime;

    installApi(callInvoker, jsRuntime);
    return @true;
  }
  return @false;
}


@end
