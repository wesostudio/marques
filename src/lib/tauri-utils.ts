import { invoke } from "@tauri-apps/api/tauri";
import initialConfig from "../../src/initialConfig.json";
import type { GlobalConfig } from "$lib/types/config.types";

export async function createConfig(config: GlobalConfig) {
  try {
    await invoke("create_config_file", { config });
    console.log("Configuration file created successfully.");
  } catch (error) {
    console.error("Error creating config file:", error);
  }
}

export function loadConfig() {
  invoke("read_config_file")
    .then((result) => {
      const parsedConfig = JSON.parse(result as string);
      return parsedConfig;
    })
    .catch(async () => {
      //@ts-ignore
      await createConfig(initialConfig);
    });
}
