#ifndef JSI_HOST_H
#define JSI_HOST_H
#pragma once

#include "rust/cxx.h"
#include "jsi/jsi.h"

namespace jsi_rs
{
  namespace ffi
  {
    using Value = ::facebook::jsi::Value;
    using HostObject = ::facebook::jsi::HostObject;
    using Runtime = ::facebook::jsi::Runtime;
    using PropNameID = ::facebook::jsi::PropNameID;
    using JSError = ::facebook::jsi::JSError;

    struct RustHostObject;

    ::std::unique_ptr<Value> rho_get(
        RustHostObject &_self, Runtime &rt, const PropNameID &name);

    void rho_set(
        RustHostObject &_self, Runtime &rt, const PropNameID &name, const Value &value);

    ::std::unique_ptr<::std::vector<PropNameID>> rho_properties(
        RustHostObject &_self, Runtime &rt) noexcept;

    class CxxHostObject : public HostObject
    {
    public:
      rust::Box<RustHostObject> inner;

      CxxHostObject(rust::Box<RustHostObject> it) : HostObject(), inner(std::move(it)) {}

      Value get(Runtime &rt, const PropNameID &name)
      {
        try
        {
          auto value = rho_get(*inner, rt, name);
          return std::move(*value.release());
        }
        catch (rust::Error &e)
        {
          throw JSError(rt, e.what());
        }
      }

      void set(Runtime &rt, const PropNameID &name, Value const &value)
      {
        try
        {
          rho_set(*inner, rt, name, value);
        }
        catch (rust::Error &e)
        {
          throw JSError(rt, e.what());
        }
      }

      std::vector<PropNameID> getPropertyNames(Runtime &rt)
      {
        auto value = rho_properties(*inner, rt);
        return std::move(*value.release());
      }
    };

    ::std::unique_ptr<CxxHostObject> CxxHostObject_create(
        rust::Box<::jsi_rs::ffi::RustHostObject> rho) noexcept
    {
      return std::make_unique<CxxHostObject>(std::move(rho));
    }

    ::std::unique_ptr<HostObject> CxxHostObject_toHostObjectU(
        ::std::unique_ptr<CxxHostObject> cho) noexcept
    {
      return cho;
    }

    ::std::unique_ptr<CxxHostObject> CxxHostObject_fromHostObjectU(
        ::std::unique_ptr<HostObject> ho) noexcept
    {
      return std::unique_ptr<CxxHostObject>(dynamic_cast<CxxHostObject *>(ho.release()));
    }

    ::std::shared_ptr<HostObject> CxxHostObject_toHostObjectS(
        ::std::shared_ptr<CxxHostObject> cho) noexcept
    {
      return cho;
    }

    ::std::shared_ptr<CxxHostObject> CxxHostObject_fromHostObjectS(
        ::std::shared_ptr<HostObject> ho) noexcept
    {
      return std::dynamic_pointer_cast<CxxHostObject>(ho);
    }

    RustHostObject const &CxxHostObject_getInner(
        CxxHostObject const &cho) noexcept
    {
      return *cho.inner;
    }

    RustHostObject &CxxHostObject_getInnerMut(
        CxxHostObject &cho) noexcept
    {
      return *cho.inner;
    }
  }
}

#endif
