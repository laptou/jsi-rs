// A wrapper header b/c a bunch of JSI functions return opaque C++ types
// directly and autocxx currently isn't able to handle these headers b/c of
// stuff like nested protected structs

#ifndef JSI_WRAPPER_H
#define JSI_WRAPPER_H
#pragma once

#include <ReactCommon/CallInvoker.h>

#include <jsi/jsi.h>
#include <rust/cxx.h>

namespace jsi_rs
{
namespace ffi
{
using Buffer = ::facebook::jsi::Buffer;
using StringBuffer = ::facebook::jsi::StringBuffer;
using PreparedJavaScript = ::facebook::jsi::PreparedJavaScript;
using Symbol = ::facebook::jsi::Symbol;
using String = ::facebook::jsi::String;
using Object = ::facebook::jsi::Object;
using WeakObject = ::facebook::jsi::WeakObject;
using Array = ::facebook::jsi::Array;
using ArrayBuffer = ::facebook::jsi::ArrayBuffer;
using Function = ::facebook::jsi::Function;
using Value = ::facebook::jsi::Value;
using Instrumentation = ::facebook::jsi::Instrumentation;
using Scope = ::facebook::jsi::Scope;
using JSIException = ::facebook::jsi::JSIException;
using JSError = ::facebook::jsi::JSError;
using HostObject = ::facebook::jsi::HostObject;
using Runtime = ::facebook::jsi::Runtime;
using Pointer = ::facebook::jsi::Pointer;
using PropNameID = ::facebook::jsi::PropNameID;

using ConstPreparedJavaScript = const PreparedJavaScript;

// std::ffi::c_void is not supported by CXX
using c_void = void;

// HostObject

::std::unique_ptr<Value> HostObject_get(
    ::facebook::jsi::HostObject &self, Runtime &rt, const PropNameID &name
) noexcept
{
  Value (::facebook::jsi::HostObject::*data$)(
      facebook::jsi::Runtime &, const facebook::jsi::PropNameID &name
  ) = &::facebook::jsi::HostObject::get;
  auto value = (self.*data$)(rt, name);
  return ::std::make_unique<Value>(std::move(value));
}

::std::unique_ptr<std::vector<PropNameID>> HostObject_getPropertyNames(
    ::facebook::jsi::HostObject &self, Runtime &rt
) noexcept
{
  std::vector<PropNameID> (::facebook::jsi::HostObject::*
                               data$)(facebook::jsi::Runtime &) =
      &::facebook::jsi::HostObject::getPropertyNames;
  auto value = (self.*data$)(rt);
  return ::std::unique_ptr<std::vector<PropNameID>>(&value);
}

// Runtime

::std::shared_ptr<ConstPreparedJavaScript>
PreparedJavaScript_asConst(const ::std::shared_ptr<PreparedJavaScript> &js
) noexcept
{
  return ::std::shared_ptr<ConstPreparedJavaScript>(js);
}

::std::unique_ptr<Value> Runtime_evaluateJavaScript(
    ::facebook::jsi::Runtime &self, const std::shared_ptr<Buffer> &buffer,
    rust::Str sourceURL
) noexcept
{
  Value (::facebook::jsi::Runtime::*data$)(
      const std::shared_ptr<const Buffer> &buffer, const std::string &sourceURL
  ) = &::facebook::jsi::Runtime::evaluateJavaScript;
  auto value = (self.*data$)(buffer, std::string(sourceURL));
  return ::std::make_unique<Value>(std::move(value));
}

::std::shared_ptr<ConstPreparedJavaScript> Runtime_prepareJavaScript(
    ::facebook::jsi::Runtime &self, const std::shared_ptr<Buffer> &buffer,
    rust::Str sourceURL
) noexcept
{
  std::shared_ptr<ConstPreparedJavaScript> (::facebook::jsi::Runtime::*data$)(
      const std::shared_ptr<const Buffer> &buffer, const std::string sourceURL
  ) = &::facebook::jsi::Runtime::prepareJavaScript;
  return (self.*data$)(buffer, std::string(sourceURL));
}

::std::unique_ptr<Value> Runtime_evaluatePreparedJavaScript(
    ::facebook::jsi::Runtime &self,
    const ::std::shared_ptr<ConstPreparedJavaScript> &js
) noexcept
{
  Value (::facebook::jsi::Runtime::*
             data$)(const ::std::shared_ptr<ConstPreparedJavaScript> &) =
      &::facebook::jsi::Runtime::evaluatePreparedJavaScript;
  auto value = (self.*data$)(::std::shared_ptr<ConstPreparedJavaScript>(js));
  return ::std::make_unique<Value>(std::move(value));
}

::std::unique_ptr<Object> Runtime_global(::facebook::jsi::Runtime &self
) noexcept
{
  Object (::facebook::jsi::Runtime::*fp)() = &::facebook::jsi::Runtime::global;
  auto value = (self.*fp)();
  return ::std::make_unique<Object>(std::move(value));
}

::std::unique_ptr<std::string>
Runtime_description(::facebook::jsi::Runtime &self) noexcept
{
  std::string (::facebook::jsi::Runtime::*fp)() =
      &::facebook::jsi::Runtime::description;
  auto value = (self.*fp)();
  return ::std::make_unique<std::string>(std::move(value));
}

// PropNameID

::std::unique_ptr<PropNameID>
PropNameID_forUtf8(Runtime &rt, rust::Str str) noexcept
{
  auto value =
      PropNameID::forUtf8(rt, (const uint8_t *)str.data(), str.length());
  return std::make_unique<PropNameID>(std::move(value));
}

::std::unique_ptr<PropNameID>
PropNameID_forString(Runtime &rt, const String &str) noexcept
{
  auto value = PropNameID::forString(rt, str);
  return std::make_unique<PropNameID>(std::move(value));
}

::std::unique_ptr<std::string>
PropNameID_toUtf8(const PropNameID &self, Runtime &rt) noexcept
{
  std::string (::facebook::jsi::PropNameID::*fp)(Runtime &rt) const =
      &::facebook::jsi::PropNameID::utf8;
  auto value = (self.*fp)(rt);
  return std::make_unique<std::string>(std::move(value));
}

bool PropNameID_compare(
    Runtime &rt, const PropNameID &self, const PropNameID &other
) noexcept
{
  return PropNameID::compare(rt, self, other);
}

std::unique_ptr<PropNameID> PropNameID_copy(const PropNameID &self, Runtime &rt)
{
  auto val = PropNameID(rt, self);
  return std::make_unique<PropNameID>(std::move(val));
}

// Symbol

::std::unique_ptr<std::string>
Symbol_toString(const Symbol &self, Runtime &rt) noexcept
{
  std::string (::facebook::jsi::Symbol::*fp)(Runtime &rt) const =
      &::facebook::jsi::Symbol::toString;
  auto value = (self.*fp)(rt);
  return std::make_unique<std::string>(std::move(value));
}

bool Symbol_compare(
    Runtime &rt, const Symbol &self, const Symbol &other
) noexcept
{
  return Symbol::strictEquals(rt, self, other);
}

// String

::std::unique_ptr<String> String_fromUtf8(Runtime &rt, rust::Str str) noexcept
{
  auto value =
      String::createFromUtf8(rt, (const uint8_t *)str.data(), str.length());
  return std::make_unique<String>(std::move(value));
}

::std::unique_ptr<std::string>
String_toString(const String &self, Runtime &rt) noexcept
{
  std::string (::facebook::jsi::String::*fp)(Runtime &rt) const =
      &::facebook::jsi::String::utf8;
  auto value = (self.*fp)(rt);
  return std::make_unique<std::string>(std::move(value));
}

bool String_compare(
    Runtime &rt, const String &self, const String &other
) noexcept
{
  return String::strictEquals(rt, self, other);
}

// Object

::std::unique_ptr<Object> Object_create(Runtime &rt) noexcept
{
  return std::make_unique<Object>(rt);
}

::std::unique_ptr<Object> Object_createFromHostObjectShared(
    Runtime &rt, std::shared_ptr<HostObject> ho
) noexcept
{
  auto value = facebook::jsi::Object::createFromHostObject(rt, ho);
  return std::make_unique<Object>(std::move(value));
}

::std::unique_ptr<Object> Object_createFromHostObjectUnique(
    Runtime &rt, std::unique_ptr<HostObject> ho
) noexcept
{
  std::shared_ptr<HostObject> s_ho = std::move(ho);
  auto value = facebook::jsi::Object::createFromHostObject(rt, s_ho);
  return std::make_unique<Object>(std::move(value));
}

bool Object_compare(
    Runtime &rt, const Object &self, const Object &other
) noexcept
{
  return Object::strictEquals(rt, self, other);
}

std::unique_ptr<Value> Object_getProperty(
    const Object &self, Runtime &rt, const PropNameID &name
) noexcept
{
  Value (::facebook::jsi::Object::*fp)(Runtime &, const PropNameID &) const =
      &::facebook::jsi::Object::getProperty;
  auto value = (self.*fp)(rt, name);
  return std::make_unique<Value>(std::move(value));
}

void Object_setProperty(
    Object &self, Runtime &rt, const PropNameID &name, Value const &value
) noexcept
{
  void (::facebook::jsi::Object::*
            fp)(Runtime &, const PropNameID &, Value const &) const =
      &::facebook::jsi::Object::setProperty;
  (self.*fp)(rt, name, value);
}

std::unique_ptr<Array> Object_asArray(Object const &self, Runtime &rt)
{
  Array (::facebook::jsi::Object::*fp)(Runtime &) const & =
      &::facebook::jsi::Object::asArray;
  auto val = (self.*fp)(rt);
  return std::make_unique<Array>(std::move(val));
}

std::unique_ptr<ArrayBuffer>
Object_asArrayBuffer(Object const &self, Runtime &rt)
{
  ArrayBuffer (::facebook::jsi::Object::*fp)(Runtime &) const & =
      &::facebook::jsi::Object::getArrayBuffer;
  auto val = (self.*fp)(rt);
  return std::make_unique<ArrayBuffer>(std::move(val));
}

std::unique_ptr<Function> Object_asFunction(Object const &self, Runtime &rt)
{
  Function (::facebook::jsi::Object::*fp)(Runtime &) const & =
      &::facebook::jsi::Object::asFunction;
  auto val = (self.*fp)(rt);
  return std::make_unique<Function>(std::move(val));
}

std::shared_ptr<HostObject> Object_asHostObject(Object const &self, Runtime &rt)
{
  std::shared_ptr<HostObject> (::facebook::jsi::Object::*fp)(Runtime &) const =
      &::facebook::jsi::Object::asHostObject;
  return (self.*fp)(rt);
}

std::unique_ptr<Array> Object_getPropertyNames(Object &self, Runtime &rt)
{
  Array (::facebook::jsi::Object::*fp)(Runtime &) const =
      &::facebook::jsi::Object::getPropertyNames;
  auto val = (self.*fp)(rt);
  return std::make_unique<Array>(std::move(val));
}

// WeakObject

std::unique_ptr<WeakObject>
WeakObject_fromObject(Runtime &rt, Object const &object)
{
  return std::make_unique<WeakObject>(rt, object);
}

std::unique_ptr<Value> WeakObject_lock(WeakObject &self, Runtime &rt)
{
  Value (::facebook::jsi::WeakObject::*fp)(Runtime &) const =
      &::facebook::jsi::WeakObject::lock;
  auto val = (self.*fp)(rt);
  return std::make_unique<Value>(std::move(val));
}

// Array

std::unique_ptr<Array> Array_createWithLength(Runtime &rt, size_t length)
{
  return std::make_unique<Array>(rt, length);
}

std::unique_ptr<Value> Array_get(Array const &self, Runtime &rt, size_t index)
{
  Value (::facebook::jsi::Array::*fp)(Runtime &, size_t) const =
      &::facebook::jsi::Array::getValueAtIndex;
  auto val = (self.*fp)(rt, index);
  return std::make_unique<Value>(std::move(val));
}

void Array_set(Array &self, Runtime &rt, size_t index, Value const &value)
{
  void (::facebook::jsi::Array::*fp)(Runtime &, size_t, Value const &) const =
      &::facebook::jsi::Array::setValueAtIndex;
  (self.*fp)(rt, index, &value);
}

// Function

std::unique_ptr<Value>
Function_call(Function const &self, Runtime &rt, std::vector<Value> const &args)
{
  Value (::facebook::jsi::Function::*fp)(Runtime &, Value const *, size_t)
      const = &::facebook::jsi::Function::call;
  auto value = (self.*fp)(rt, args.data(), args.size());
  return std::make_unique<Value>(std::move(value));
}

std::unique_ptr<Value> Function_callWithThis(
    Function const &self, Runtime &rt, Object const &thisObj,
    std::vector<Value> const &args
)
{
  Value (::facebook::jsi::Function::*fp)(
      Runtime &, Object const &, Value const *, size_t
  ) const = &::facebook::jsi::Function::callWithThis;
  auto value = (self.*fp)(rt, thisObj, args.data(), args.size());
  return std::make_unique<Value>(std::move(value));
}

std::unique_ptr<Value> Function_callAsConstructor(
    Function const &self, Runtime &rt, std::vector<Value> const &args
)
{
  Value (::facebook::jsi::Function::*fp)(Runtime &, Value const *, size_t)
      const = &::facebook::jsi::Function::callAsConstructor;
  auto value = (self.*fp)(rt, args.data(), args.size());
  return std::make_unique<Value>(std::move(value));
}

::std::unique_ptr<::facebook::jsi::Value> host_fn_trampoline(
    ::facebook::jsi::Runtime &rt, const ::facebook::jsi::Value &thisVal,
    const ::facebook::jsi::Value *args, ::std::uint32_t count,
    ::std::size_t stride, ::jsi_rs::ffi::c_void *closure
);

std::unique_ptr<Function> Function_createFromHostFunction(
    Runtime &rt, const PropNameID &name, unsigned int paramCount, void *closure
)
{
  auto value = Function::createFromHostFunction(
      rt, name, paramCount,
      [closure](
          Runtime &rt, const Value &thisVal, const Value *args, size_t count
      ) {
        auto stride = sizeof(Value);
        auto val =
            host_fn_trampoline(rt, thisVal, args, count, stride, closure);
        return std::move(*val.release());
      }
  );
  return std::make_unique<Function>(std::move(value));
}

// Value

std::unique_ptr<Value> Value_fromUndefined()
{
  return std::make_unique<Value>();
}

std::unique_ptr<Value> Value_fromNull()
{
  return std::make_unique<Value>(std::nullptr_t());
}

std::unique_ptr<Value> Value_fromInt(int i)
{
  return std::make_unique<Value>(i);
}

std::unique_ptr<Value> Value_fromBool(bool b)
{
  return std::make_unique<Value>(b);
}

std::unique_ptr<Value> Value_fromDouble(double d)
{
  return std::make_unique<Value>(d);
}

std::unique_ptr<Value> Value_fromString(Runtime &rt, std::unique_ptr<String> s)
{
  return std::make_unique<Value>(rt, std::move(*s.release()));
}

std::unique_ptr<Value> Value_fromSymbol(Runtime &rt, std::unique_ptr<Symbol> s)
{
  return std::make_unique<Value>(rt, std::move(*s.release()));
}

std::unique_ptr<Value> Value_fromObject(Runtime &rt, std::unique_ptr<Object> o)
{
  return std::make_unique<Value>(rt, std::move(*o.release()));
}

std::unique_ptr<Value> Value_copyFromString(Runtime &rt, const String &s)
{
  return std::make_unique<Value>(rt, s);
}

std::unique_ptr<Value> Value_copyFromSymbol(Runtime &rt, const Symbol &s)
{
  return std::make_unique<Value>(rt, s);
}

std::unique_ptr<Value> Value_copyFromObject(Runtime &rt, const Object &o)
{
  return std::make_unique<Value>(rt, o);
}

std::unique_ptr<Value> Value_fromJson(Runtime &rt, rust::Str s)
{
  auto val =
      Value::createFromJsonUtf8(rt, (const uint8_t *)s.data(), s.length());
  return std::make_unique<Value>(std::move(val));
}

bool Value_compare(Runtime &rt, const Value &self, const Value &other) noexcept
{
  return Value::strictEquals(rt, self, other);
}

std::unique_ptr<String> Value_asString(const Value &self, Runtime &rt)
{
  String (::facebook::jsi::Value::*fp)(Runtime &) const & =
      &::facebook::jsi::Value::asString;
  auto val = (self.*fp)(rt);
  return std::make_unique<String>(std::move(val));
}

std::unique_ptr<Symbol> Value_asSymbol(const Value &self, Runtime &rt)
{
  Symbol (::facebook::jsi::Value::*fp)(Runtime &) const & =
      &::facebook::jsi::Value::asSymbol;
  auto val = (self.*fp)(rt);
  return std::make_unique<Symbol>(std::move(val));
}

std::unique_ptr<Object> Value_asObject(const Value &self, Runtime &rt)
{
  Object (::facebook::jsi::Value::*fp)(Runtime &) const & =
      &::facebook::jsi::Value::asObject;
  auto val = (self.*fp)(rt);
  return std::make_unique<Object>(std::move(val));
}

std::unique_ptr<String> Value_toString(const Value &self, Runtime &rt)
{
  String (::facebook::jsi::Value::*fp)(Runtime &) const =
      &::facebook::jsi::Value::toString;
  auto val = (self.*fp)(rt);
  return std::make_unique<String>(std::move(val));
}

std::unique_ptr<Value> Value_copy(const Value &self, Runtime &rt)
{
  auto val = Value(rt, self);
  return std::make_unique<Value>(std::move(val));
}

// CallInvoker

void call_invoker_trampoline(void *closure);

void CallInvoker_invokeSync(
    std::shared_ptr<facebook::react::CallInvoker> ci, void *closure
)
{
  ci->invokeSync([closure]() { call_invoker_trampoline(closure); });
}

void CallInvoker_invokeAsync(
    std::shared_ptr<facebook::react::CallInvoker> ci, void *closure
)
{
  ci->invokeAsync([closure]() { call_invoker_trampoline(closure); });
}

// Utility fns

std::unique_ptr<std::vector<Value>> create_value_vector()
{
  return std::make_unique<std::vector<Value>>();
}

void push_value_vector(std::vector<Value> &values, std::unique_ptr<Value> value)
{
  auto x = std::move(*value.release());
  values.push_back(std::move(x));
}

std::unique_ptr<std::vector<PropNameID>> create_prop_name_vector()
{
  return std::make_unique<std::vector<PropNameID>>();
}

void push_prop_name_vector(
    std::vector<PropNameID> &values, std::unique_ptr<PropNameID> value
)
{
  auto x = std::move(*value.release());
  values.push_back(std::move(x));
}

std::unique_ptr<PropNameID> pop_prop_name_vector(std::vector<PropNameID> &values
)
{
  if (values.size() > 0) {
    auto prop = std::move(values.back());
    values.pop_back();
    return std::make_unique<PropNameID>(std::move(prop));
  } else {
    return std::unique_ptr<PropNameID>(nullptr);
  }
}
} // namespace ffi
} // namespace jsi_rs

#endif
