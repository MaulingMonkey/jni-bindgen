use super::*;

pub trait AsLocal<Class: AsValidJObjectAndEnv> {
    fn as_local<'env>(self, env: &'env Env) -> Local<'env, Class>;
}

pub trait AsOptionalLocal<Class: AsValidJObjectAndEnv> {
    fn as_optional_local<'env>(self, env: &'env Env) -> Option<Local<'env, Class>>;
}
