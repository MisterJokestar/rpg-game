import type { Config } from "vike/types";
import vikeReact from "vike-react/config";

// Default config (can be overridden by pages)
// https://vike.dev/config

const config: Config = {
  // https://vike.dev/head-tags
  title: "Cloud Final Project",
  description: "Game utalizing Cloud Resources",
  prerender: true,
  extends: [vikeReact],
};

export default config;
