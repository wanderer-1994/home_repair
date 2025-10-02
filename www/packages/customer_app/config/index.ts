import LocalConfig from "@/config/config.local.json";
import ProdConfig from "@/config/config.prod.json";
import { MergeTuple } from "@/config/merge_config";

// 1. Define a consistent type structure
export type AppConfig = MergeTuple<[typeof ProdConfig, typeof LocalConfig]>;

// 2. Determine the current environment from an EXPO_PUBLIC variable
// Metro and Expo support injecting variables prefixed with 'EXPO_PUBLIC_'
const ENV = process.env.EXPO_PUBLIC_APP_ENV || "local";
console.log(`Loading config for environment: ${ENV}`);

let Config: AppConfig;

switch (ENV) {
  case "local":
    Config = LocalConfig as AppConfig;
    break;
  case "prod":
  default:
    Config = ProdConfig as AppConfig;
    break;
}

export default Config;
