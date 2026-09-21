import wasmPackage from "@spensercai/qimen-wasm/package.json";

export const ENGINE_VERSION: string = wasmPackage.version;
export const ENGINE_ASSET_DIRECTORY = `/wasm/qimen-${ENGINE_VERSION}/`;
