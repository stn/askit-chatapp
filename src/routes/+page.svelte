<script lang="ts">
  import IconSend from "@lucide/svelte/icons/send-horizontal";
  import { Avatar } from "bits-ui";
  import { onDestroy, onMount } from "svelte";
  import "../app.css";

  import { listen } from '@tauri-apps/api/event';
  import { writeBoard } from "tauri-plugin-askit-api";

  import type { Message, BoardMessage } from "../types/chat";

  // Types
  interface Person {
    id: number;
    avatar: number;
    name: string;
  }

  interface MessageFeed {
    id: number;
    host: boolean;
    avatar: number;
    name: string;
    timestamp: string;
    message: string;
    color: string;
  }

  let elemChat: HTMLElement;
  const lorem =
    "Lorem, ipsum dolor sit amet consectetur adipisicing elit. Provident blanditiis quidem dolorum ab similique. Voluptatibus quibusdam unde mollitia corrupti assumenda libero. Quibusdam culpa illum unde asperiores accusantium! Unde, cupiditate tenetur.";

  // Navigation List
  const people: Person[] = [
    { id: 0, avatar: 14, name: "Michael" },
    { id: 1, avatar: 40, name: "Janet" },
    { id: 2, avatar: 31, name: "Susan" },
    { id: 3, avatar: 56, name: "Joey" },
    { id: 4, avatar: 24, name: "Lara" },
    { id: 5, avatar: 9, name: "Melissa" },
  ];
  let currentPersonId: number = people[0].id;

  // Messages
  let messageFeed: MessageFeed[] = [
    {
      id: 0,
      host: true,
      avatar: 48,
      name: "Jane",
      timestamp: "Yesterday @ 2:30pm",
      message: lorem,
      color: "preset-tonal-primary",
    },
    {
      id: 1,
      host: false,
      avatar: 14,
      name: "Michael",
      timestamp: "Yesterday @ 2:45pm",
      message: lorem,
      color: "preset-tonal-primary",
    },
    {
      id: 2,
      host: true,
      avatar: 48,
      name: "Jane",
      timestamp: "Yesterday @ 2:50pm",
      message: lorem,
      color: "preset-tonal-primary",
    },
    {
      id: 3,
      host: false,
      avatar: 14,
      name: "Michael",
      timestamp: "Yesterday @ 2:52pm",
      message: lorem,
      color: "preset-tonal-primary",
    },
  ];
  let currentMessage = "";

  function scrollChatBottom(behavior?: "auto" | "instant" | "smooth") {
    elemChat.scrollTo({ top: elemChat.scrollHeight, behavior });
  }

  function getCurrentTimestamp(): string {
    return new Date().toLocaleString("en-US", {
      hour: "numeric",
      minute: "numeric",
      hour12: true,
    });
  }

  function addMessage() {
    const newMessage = {
      id: messageFeed.length,
      host: true,
      avatar: 48,
      name: "Jane",
      timestamp: `Today @ ${getCurrentTimestamp()}`,
      message: currentMessage,
      color: "preset-tonal-primary",
    };
    // Update the message feed
    messageFeed = [...messageFeed, newMessage];
    // Write to board
    console.log("Writing to board:", currentMessage);
    writeBoard("user_message", currentMessage);
    // Clear prompt
    currentMessage = "";
    // Smooth scroll to bottom
    // Timeout prevents race condition
    setTimeout(() => scrollChatBottom("smooth"), 0);
  }

  function onPromptKeydown(event: KeyboardEvent) {
    if (["Enter"].includes(event.code) && event.ctrlKey) {
      event.preventDefault();
      addMessage();
    }
  }

  let unlistenBoard: () => void;

  // When DOM is mounted, scroll to bottom
  onMount(() => {
    listen<BoardMessage>('notify_board', (event) => {
      console.log('Received board message:', event);
      const boardMessage = event.payload;
      if (boardMessage?.name !== "assistant_message" || !boardMessage?.data?.value?.content) {
        return;
      }
      const newMessage = {
        id: messageFeed.length,
        host: false,
        avatar: 14,
        name: "Michael",
        timestamp: `Today @ ${getCurrentTimestamp()}`,
        message: boardMessage.data.value.content,
        color: "preset-tonal-primary",
      };
      // Update the message feed
      if (messageFeed.length > 0 && messageFeed[messageFeed.length - 1].host === false) {
        const updated = [...messageFeed];
        updated[updated.length - 1].message = boardMessage.data.value.content;
        messageFeed = updated;
      } else {
        messageFeed = [...messageFeed, newMessage];
      }

      // setMessages((prev) => {
      //   if (prev.length > 0) {
      //     const updated = [...prev];
      //     updated[updated.length - 1].content = boardMessage.data.value.content;
      //     return updated;
      //   } else {
      //     return prev;
      //   }
      // });
      setTimeout(() => scrollChatBottom("smooth"), 0);
    }).then((unlistenFn) => {
      unlistenBoard = unlistenFn;
    });

    scrollChatBottom();
  });

  onDestroy(() => {
    if (unlistenBoard) {
      unlistenBoard();
    }
  });

  // let name = $state("");
  // let greetMsg = $state("");

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }
</script>

<main class="container w-screen h-screen">
  <section class="w-screen h-screen">
    <div class="chat w-screen h-screen flex flex-row">
      <!-- Navigation -->
      <div
        class="hidden flex-none lg:grid grid-rows-[auto_1fr_auto] border-r-[1px] border-surface-200-800"
      >
        <!-- Header -->
        <header class="border-b-[1px] border-surface-200-800 p-4">
          <input class="input" type="search" placeholder="Search..." />
        </header>
        <!-- List -->
        <div class="p-4 space-y-4 overflow-y-auto">
          <small class="opacity-50">Contacts</small>
          <div class="space-y-1">
            {#each people as person}
              <button
                type="button"
                class="card p-2 w-full flex items-center space-x-4 {person.id ===
                currentPersonId
                  ? 'preset-filled-primary-500'
                  : 'bg-surface-hover-token'}"
                onclick={() => (currentPersonId = person.id)}
              >
                <Avatar.Root
                  class="size-8"
                  >
                  <Avatar.Image
                    src="https://i.pravatar.cc/?img={person.avatar}"
                    alt={person.name}
                  />
                </Avatar.Root>
                <span class="flex-1 text-start">
                  {person.name}
                </span>
              </button>
            {/each}
          </div>
        </div>
        <!-- Footer -->
        <!-- <footer class="border-t-[1px] border-surface-200-800 p-4">(footer)</footer> -->
      </div>
      <!-- Chat -->
      <div class="flex-auto grid grid-rows-[1fr_auto]">
        <!-- Conversation -->
        <section
          bind:this={elemChat}
          class="p-4 overflow-y-auto space-y-4 w-full h-full"
        >
          {#each messageFeed as bubble}
            {#if bubble.host === true}
              <div class="flex flex-row-reverse items-start gap-2">
                <Avatar.Root>
                  <Avatar.Image
                    src="https://i.pravatar.cc/?img={bubble.avatar}"
                    alt={bubble.name}
                    class="size-12"
                  />
                </Avatar.Root>
                <div class="flex-1 flex justify-end">
                  <div class="card pr-2 pl-4 preset-tonal rounded-tl-none space-y-2">
                    <!-- <header class="flex flex-col justify-start items-end">
                      <p class="font-bold">{bubble.name}</p>
                    </header> -->
                    <p class="rounded-lg bg-blue-100 p-2">{bubble.message}</p>
                    <!-- <footer class="flex flex-col justify-start items-end">
                      <small class="opacity-50">{bubble.timestamp}</small>
                    </footer> -->
                  </div>
                </div>
              </div>
            {:else}
              <div class="flex flex-row items-start gap-2">
                <Avatar.Root>
                  <Avatar.Image
                    src="https://i.pravatar.cc/?img={bubble.avatar}"
                    alt={bubble.name}
                    class="size-12"
                  />
                </Avatar.Root>
                <div class="flex-1 flex justify-start">
                  <div class="card pl-2 pr-4 rounded-tr-none space-y-2 {bubble.color}">
                    <!-- <header class="flex flex-col justify-start items-start">
                      <p class="font-bold">{bubble.name}</p>
                      <small class="opacity-50">{bubble.timestamp}</small>
                    </header> -->
                    <p class="rounded-lg bg-gray-100 p-2">{bubble.message}</p>
                  </div>
                </div>
              </div>
            {/if}
          {/each}
        </section>
        <!-- Prompt -->
        <section class="p-4">
          <div
            class="flex flex-row input-group divide-x divide-surface-200-800 rounded-container-token"
          >
            <textarea
              value={currentMessage}
              oninput={(e) => (currentMessage = e.currentTarget.value)}
              class="flex-1 bg-transparent border-1 p-2 rounded-lg"
              name="prompt"
              id="prompt"
              placeholder="Write a message..."
              rows="2"
              onkeydown={onPromptKeydown}
            ></textarea>
            <button
              class="pl-2 flex-none input-group-cell {currentMessage
                ? 'preset-filled-primary-500'
                : 'preset-tonal'}"
              onclick={addMessage}
            >
              <IconSend />
            </button>
          </div>
        </section>
      </div>
    </div>
  </section>
</main>
