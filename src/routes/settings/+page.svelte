<script lang="ts">
  import IconSettings from "@lucide/svelte/icons/settings";
  import IconEye from "@lucide/svelte/icons/eye";
  import IconEyeOff from "@lucide/svelte/icons/eye-off";
  import { onMount } from "svelte";
  import { Store } from "@tauri-apps/plugin-store";
  import type { Settings } from "$lib/types/settings";
  import { defaultSettings } from "$lib/types/settings";
  import { loadSettingsCmd } from "$lib/commands.ts";

  let store: Store | null = $state(null);
  let settings = $state<Settings>(defaultSettings);
  let isLoading = $state(false);
  let savedMessage = $state("");
  let visibleKeys = $state<{ [key: string]: boolean }>({});

  async function getStore() {
    if (!store) {
      store = await Store.load("settings.json");
    }
    return store;
  }

  async function loadSettings() {
    try {
      const storeInstance = await getStore();
      const loadedSettings = await storeInstance.get<Settings>("settings");

      if (loadedSettings) {
        settings = {
          ...loadedSettings,
          apiKeys: loadedSettings.apiKeys || {},
        };
      } else {
        settings = defaultSettings;
      }
    } catch (error) {
      console.error("設定の読み込みに失敗:", error);
      settings = defaultSettings;
    }
  }

  async function saveSettings() {
    isLoading = true;
    savedMessage = "";
    try {
      const storeInstance = await getStore();
      await storeInstance.set("settings", settings);
      await storeInstance.save();
      await loadSettingsCmd();
      savedMessage = "設定を保存しました";
      setTimeout(() => (savedMessage = ""), 3000);
    } catch (error) {
      console.error("設定の保存に失敗:", error);
      savedMessage = "設定の保存に失敗しました";
    } finally {
      isLoading = false;
    }
  }

  function updateApiKey(service: string, value: string) {
    settings = {
      ...settings,
      apiKeys: {
        ...(settings.apiKeys || {}),
        [service]: value,
      },
    };
  }

  function toggleKeyVisibility(service: string) {
    visibleKeys = {
      ...visibleKeys,
      [service]: !visibleKeys[service],
    };
  }

  const apiServices = [
    { key: "openai", label: "OpenAI API Key", placeholder: "sk-..." },
    { key: "sakura", label: "Sakura AI API Key", placeholder: "sk-..." },
    { key: "ollamaUrl", label: "Ollama URL", placeholder: "http://localhost:11434" },
  ];

  onMount(() => {
    loadSettings();
  });
</script>

<div class="flex-auto h-screen bg-background overflow-auto">
  <div class="mx-auto p-6 space-y-6">
    <div class="flex items-center gap-3 mb-6">
      <IconSettings class="h-8 w-8" />
      <h1 class="text-3xl font-bold">設定</h1>
    </div>

    <!-- Tabs -->
    <div class="w-full">
      <!-- API Tab -->
        <div class="space-y-4 mt-6">
          <div class="card p-6">
            <div class="mb-4">
              <h2 class="text-2xl font-semibold">API キー管理</h2>
              <p class="text-sm text-muted-foreground mt-1">
                外部サービスのAPIキーを設定してください。
              </p>
            </div>
            <div class="space-y-4">
              {#each apiServices as service}
                <div class="space-y-2">
                  <label for={service.key} class="text-sm font-medium">
                    {service.label}
                  </label>
                  <div class="relative">
                    <input
                      id={service.key}
                      type={visibleKeys[service.key] ? "text" : "password"}
                      value={settings.apiKeys?.[service.key] || ""}
                      oninput={(e) => updateApiKey(service.key, e.currentTarget.value)}
                      placeholder={service.placeholder}
                      class="w-full px-3 py-2 pr-10 bg-transparent border border-surface-400-600 rounded-md"
                    />
                    <button
                      type="button"
                      class="absolute right-0 top-0 h-full px-3 hover:bg-surface-hover rounded-r-md"
                      onclick={() => toggleKeyVisibility(service.key)}
                    >
                      {#if visibleKeys[service.key]}
                        <IconEyeOff class="h-4 w-4" />
                      {:else}
                        <IconEye class="h-4 w-4" />
                      {/if}
                    </button>
                  </div>
                </div>
              {/each}

              <div class="pt-4 text-sm text-muted-foreground">
                <p>💡 APIキーの取得方法:</p>
                <ul class="list-disc list-inside space-y-1 mt-2">
                  <li>
                    <a
                      href="https://platform.openai.com/api-keys"
                      target="_blank"
                      rel="noopener noreferrer"
                      class="text-primary-500 hover:underline"
                    >
                      OpenAI API Keys
                    </a>
                  </li>
                  <li>
                    <a
                      href="https://secure.sakura.ad.jp/ai/account-tokens"
                      target="_blank"
                      rel="noopener noreferrer"
                      class="text-primary-500 hover:underline"
                    >
                      Sakura AI Engine アカウントトークン
                    </a>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
    </div>

    <div class="flex items-center justify-between">
      <div class="text-sm text-green-600 font-medium">
        {savedMessage}
      </div>
      <button
        class="px-4 py-2 bg-primary-500 text-white rounded-md hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed"
        onclick={saveSettings}
        disabled={isLoading}
      >
        {isLoading ? "保存中..." : "設定を保存"}
      </button>
    </div>
  </div>
</div>
