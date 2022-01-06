const pkg = import("../pkg");
import Widget from "./Widget";
export const ui = Widget;
export const processor = pkg;
export const name = () => "{{project-name}}";
export const description = () => "{{project-description}}";
export const process = async (config, image) => {
  return import("../pkg").then((processor) => processor.process(config, image));
};
export const defaultConfig = async () => {
  return {};
};
