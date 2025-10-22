<script lang="ts">
  import DOMPurify from "dompurify";
  import IconSend from "@lucide/svelte/icons/send-horizontal";
  import IconCat from "@lucide/svelte/icons/cat";
  import IconCircleUserRound from "@lucide/svelte/icons/circle-user-round";
  import { marked } from "marked";
    import type { MessageFeed } from "$lib/types/chat";

  interface Props {
    message: MessageFeed
  }

  let { message }: Props = $props();

  let msg = $derived.by(() => {
    return {
        id: message.id,
        host: message.host,
        timestamp: message.timestamp,
        message: marked.parse(DOMPurify.sanitize(message.message)),
        color: message.color
    }
  })
</script>

{#if msg.host === true}
    <div class="flex flex-row-reverse items-start gap-2">
        <IconCircleUserRound />
        <div class="flex-1 flex justify-end">
            <div class="card pr-2 pl-4 preset-tonal rounded-tl-none space-y-2">
                <p class="p-2">{@html msg.message}</p>
            </div>
        </div>
    </div>
{:else}
    <div class="flex flex-row items-start gap-2">
        <IconCat />
        <div class="flex-1 flex justify-start">
            <div
                class="card pl-2 pr-4 rounded-tr-none space-y-2 {msg.color}"
            >
                <p class="p-2">{@html msg.message}</p>
            </div>
        </div>
    </div>
{/if}
