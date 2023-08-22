#include <hermes/hermes.h>
// #include <hermes/CompileJS.h>
#include <hermes/Public/RuntimeConfig.h>
#include <jsi/jsi.h>
#include "rust/cxx.h"

std::unique_ptr<facebook::jsi::Runtime> cast_hermes_runtime(std::unique_ptr<facebook::hermes::HermesRuntime> runtime)
{
  return runtime;
}

std::unique_ptr<hermes::vm::RuntimeConfig> create_runtime_config()
{
  return std::make_unique<hermes::vm::RuntimeConfig>();
}

std::unique_ptr<facebook::jsi::Value> eval_js(facebook::jsi::Runtime& rt, rust::Str js)
{
  // std::string bytecode;
  // assert(hermes::compileJS(std::string(js), bytecode));
  auto out = rt.evaluateJavaScript(
      std::make_unique<facebook::jsi::StringBuffer>(std::string(js)),
      "<evaluated javascript>");
  return std::make_unique<facebook::jsi::Value>(std::move(out));
}
