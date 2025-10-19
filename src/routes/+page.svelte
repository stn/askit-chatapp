<script lang="ts">
  import IconSend from "@lucide/svelte/icons/send-horizontal";
  import IconCat from "@lucide/svelte/icons/cat";
  import IconCircleUserRound from "@lucide/svelte/icons/circle-user-round";
  import { onDestroy, onMount } from "svelte";
  import "../app.css";

  import { listen } from '@tauri-apps/api/event';
  import { writeBoard } from "tauri-plugin-askit-api";

  import type { BoardMessage } from "../types/chat";

  // Types
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

  // Messages
  let messageFeed: MessageFeed[] = [];
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
</script>

<main class="container w-screen h-screen">
  <section class="w-screen h-screen">
    <div class="chat w-screen h-screen flex flex-row">
      <!-- Navigation -->
      <div
        class="hidden flex-none lg:grid grid-rows-[auto_1fr_auto] border-r-[1px] border-surface-200-800"
      >
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
                <IconCircleUserRound />
                <div class="flex-1 flex justify-end">
                  <div class="card pr-2 pl-4 preset-tonal rounded-tl-none space-y-2">
                    <p class="rounded-lg bg-blue-100 p-2">{bubble.message}</p>
                  </div>
                </div>
              </div>
            {:else}
              <div class="flex flex-row items-start gap-2">
                <IconCat />
                <div class="flex-1 flex justify-start">
                  <div class="card pl-2 pr-4 rounded-tr-none space-y-2 {bubble.color}">
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
