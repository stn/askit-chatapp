export interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: Date;
}

export interface BoardMessage {
  name: string;
  data: any;
}
