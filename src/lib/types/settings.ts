export interface Settings {
  profile: {
    username: string;
  };
  apiKeys?: {
    [key: string]: string;
  };
}

export const defaultSettings: Settings = {
  profile: {
    username: "",
  },
  apiKeys: {},
};
