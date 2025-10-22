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

export interface MessageFeed {
  id: number;
  host: boolean;
  timestamp: string;
  message: string;
  color: string;
}
