interface Config {
  apiBaseUrl: string;
}

const config: Config = {
  apiBaseUrl: process.env.API_BASE_URL ?? "http://localhost:3001",
};

export default config;
