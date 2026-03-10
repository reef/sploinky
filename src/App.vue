<script setup lang="ts">
import VueMarkdown from "vue-markdown-render";
import { ref } from "vue";
import { getMatches } from "@tauri-apps/plugin-cli";
import { exists, readTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

const file = ref("");
const markdown = ref("");

listen("prep", () => {});

async function loadFile(path: string) {
  const fileExists = await exists(path, {
    baseDir: BaseDirectory.Home,
  });

  if (fileExists) {
    const content = await readTextFile(path, {
      baseDir: BaseDirectory.Home,
    });
    file.value = path;
    markdown.value = content;
  }
}

function closeFile() {
  file.value = "";
  markdown.value = "";
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
      <button @click="openFile()">Open</button>
      <button @click="closeFile()">Close</button>
      <code>{{ file }}</code>
    </header>
    <main>
      <vue-markdown :source="markdown" v-if="markdown" />
      <div v-else>
        <h3>Open up a Markdown file using the button above</h3>
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
  padding: 4px;
  background: #cec7d2;
  font-size: 0.8em;
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
