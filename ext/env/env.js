const { ops } = globalThis.__bootstrap.core;

class PowerEnv {
  setEnv(key, value) {
    ops.op_set_env(key, value);
  }

  getEnv(key) {
    return ops.op_get_env(key) ?? undefined;
  }

  deleteEnv(key) {
    ops.op_delete_env(key);
  }
}

const powerEnvInstance = new PowerEnv();

const POWERBASE_ENV = {
  get: powerEnvInstance.getEnv,
  toObject() {
    return ops.op_env();
  },
  set: powerEnvInstance.setEnv,
  has(key) {
    return powerEnvInstance.getEnv(key) !== undefined;
  },
  delete: powerEnvInstance.deleteEnv,
};

export { POWERBASE_ENV };
