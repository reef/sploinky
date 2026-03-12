<script setup lang="ts">
import VueMarkdown from "vue-markdown-render";
import { ref } from "vue";
import { getMatches } from "@tauri-apps/plugin-cli";
import { exists, readTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

interface Tab {
  path: string;
  content: string;
}

const activeTab = ref<Tab | null>();
const tabs = ref<Tab[]>([]);

const recent = ref<string[]>([]);
const history = localStorage.getItem("recent");
if (history) {
  try {
    const paths = JSON.parse(history);
    if (Array.isArray(paths)) recent.value = paths;
  } catch {}
}

function addRecent(path: string) {
  recent.value = [path, ...recent.value.filter((p) => p !== path).slice(0, 9)];
  localStorage.setItem("recent", JSON.stringify(recent.value));
}

async function changeTab(tab: Tab) {
  activeTab.value = tab;
}

async function closeTab(tab: Tab) {
  const i = tabs.value.indexOf(tab);
  tabs.value = [...tabs.value.slice(0, i), ...tabs.value.slice(i + 1)];
  activeTab.value = tabs.value[Math.max(0, i - 1)] ?? null;
}

async function reloadTab(tab: Tab) {
  void loadFile(tab.path);
}

listen("prep", () => {});

async function loadFile(path: string) {
  const fileExists = await exists(path, {
    baseDir: BaseDirectory.Home,
  });

  if (fileExists) {
    const content = await readTextFile(path, {
      baseDir: BaseDirectory.Home,
    });
    let existingTab = tabs.value.find((t) => t.path === path);
    if (existingTab) {
      existingTab.content = content;
      activeTab.value = existingTab;
    } else {
      const tab: Tab = {
        path,
        content,
      };
      activeTab.value = tab;
      tabs.value = [...tabs.value, tab];
    }
    addRecent(path);
  }
}

async function openFile() {
  const file = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: "Markdown files",
        extensions: ["md"],
      },
    ],
  });

  if (file) void loadFile(file);
}

async function loadArgs() {
  await listen<string[]>("got-files", (event) => {
    if (event.payload) {
      const files = event.payload;
      console.log(`Files: ${JSON.stringify(files)}`);
      if (files.length > 0) void loadFile(files[0]);
    }
  });
  invoke("window_ready");

  const args = (await getMatches()).args;

  if (args.source.value && typeof args.source.value === "string") {
    void loadFile(args.source.value);
  }
}

void loadArgs();

document.body.addEventListener(
  "click",
  function (e: Event) {
    if (!e.target) return;
    const a = e.target as HTMLAnchorElement;

    if (a?.nodeName?.toUpperCase() === "A" && a.href) {
      a.target = "_blank";
    }
  },
  true,
);
</script>

<template>
  <div class="container">
    <header>
      <div class="file-controls">
        <button @click="openFile()">Open</button>
        <code class="header-path">{{ activeTab?.path }}</code>
        <div v-if="activeTab">
          <button @click="reloadTab(activeTab)">⟳</button>
          <button @click="closeTab(activeTab)">✕</button>
        </div>
      </div>
      <div class="tab-container">
        <div
          class="tab"
          :class="{ selected: tab === activeTab }"
          v-for="tab in tabs"
          @click="changeTab(tab)"
        >
          {{ tab.path.split("/").slice(-1)[0] }}
        </div>
      </div>
    </header>
    <main>
      <vue-markdown :source="activeTab.content" v-if="activeTab" />
      <div v-else>
        <h3>Open up a Markdown file using the button above</h3>
        <div v-for="path in recent">
          <button @click="loadFile(path)">{{ path }}</button>
        </div>
      </div>
    </main>
  </div>
</template>

<style>
body {
  margin: 0;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  padding: 0;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;

  overflow: hidden;
}

.container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: stretch;
}

header {
  background: #cec7d2;
  font-size: 0.8em;
}

.header-path {
  flex-grow: 1;
  flex-shrink: 1;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-controls {
  padding: 4px;
  display: flex;
  align-items: center;
  position: sticky;
  top: 0;
}

main {
  flex-grow: 1;
  padding: 8px;
  overflow: auto;
  max-height: 100%;
}

code {
  color: #454449;
}

a {
  font-weight: 500;
  color: #8864ff;
  text-decoration: inherit;
}

a:hover {
  color: #a553f2;
}

button {
  border-radius: 2px;
  border: 1px solid transparent;
  padding: 0.2em 0.4em;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition:
    background-color 200ms,
    border-color 200ms;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  font-size: 1em;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #7639d8;
}

button:active {
  border-color: #7e29c4;
  background-color: #e8e8e8;
}

button + * {
  margin-left: 4px;
}

.tab-container {
  display: flex;
  overflow: auto;
  scrollbar-width: none;
}

.tab {
  padding: 2px 8px;
  min-width: 40px;
  cursor: pointer;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tab:hover {
  background: #8888;
}

.tab.selected {
  background: #8884;
}

@media (prefers-color-scheme: dark) {
  * {
    scrollbar-color: rgb(84, 70, 107) #2f2f2f;
    scrollbar-width: thin;
  }

  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #8b5acb;
  }

  header {
    background: #433d4b;
  }

  code {
    color: #957ab5;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
