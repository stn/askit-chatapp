export interface Settings {
  apiKeys?: {
    [key: string]: string;
  };
}

export const defaultSettings: Settings = {
  apiKeys: {},
};
