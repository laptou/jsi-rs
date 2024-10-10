
#ifndef ExampleJsiModule_h
#define ExampleJsiModule_h

#include <memory>

extern "C" {
  void ExampleJsiModule_init(void* rt, std::shared_ptr<void> ctx);
}

#endif
