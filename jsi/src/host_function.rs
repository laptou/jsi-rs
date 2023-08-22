use crate::{JsiValue, RuntimeHandle};

pub type UserHostFunction<'rt> = dyn FnMut(
        JsiValue<'rt>,           // this
        Vec<JsiValue<'rt>>,      // args
        &mut RuntimeHandle<'rt>, // runtime
    ) -> Result<JsiValue<'rt>, anyhow::Error>
    + 'rt;
