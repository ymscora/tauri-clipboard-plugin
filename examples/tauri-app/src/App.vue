<script setup lang="ts">
import { onBeforeUnmount, reactive, ref } from 'vue';
import {
  availableFormats,
  clear,
  getFilePath,
  hasFiles,
  hasFormat,
  hasHtml,
  hasImage,
  hasRtf,
  hasText,
  onClipboardChange,
  readBuffer,
  readClipboard,
  readFiles,
  readHtml,
  readImage,
  readRtf,
  readText,
  startWatch,
  stopWatch,
  writeBuffer,
  writeFiles,
  writeHtml,
  writeImage,
  writeImageBytes,
  writeRtf,
  writeText,
  writeVideoFiles,
  type ReadImageOptions,
} from '@clipboard-api';

const output = ref('');
const logs = ref<string[]>([]);
const watchStarted = ref(false);

let unlistenClipboard: null | (() => void | Promise<void>) = null;

const form = reactive({
  hasFormatName: 'image/png',
  readBufferFormat: 'application/x-demo-bin',
  writeTextValue: 'Clipboard Pro text demo',
  writeRtfValue: '{\\rtf1\\ansi\\b Clipboard Pro RTF Demo}',
  writeHtmlValue: '<div><b>Clipboard Pro HTML</b><p>with paragraph</p></div>',
  writeImagePath: '',
  imageBytesFastOnly: false,
  writeFilesText: '/tmp/demo-a.txt\n/tmp/demo-b.png',
  writeVideoFilesText: '/tmp/demo-video.mp4\n/tmp/demo-video.mov',
  writeBufferFormat: 'application/x-demo-bin',
  writeBufferDataHex: '01 02 03 04 05',
  readImageIncludeBytes: false,
  readImageAutoSave: false,
  readImagePreferRaw: true,
  writeImageAlsoStandard: false,
});

function nowTime() {
  return new Date().toLocaleTimeString();
}

function appendLog(message: string, payload?: unknown) {
  const line = payload === undefined ? `[${nowTime()}] ${message}` : `[${nowTime()}] ${message}\n${safeJson(payload)}`;
  logs.value.unshift(line);
  if (logs.value.length > 80) {
    logs.value.length = 80;
  }
}

function setOutput(value: unknown) {
  output.value = safeJson(value);
}

function safeJson(value: unknown) {
  try {
    return JSON.stringify(value, null, 2);
  } catch {
    return String(value);
  }
}

function parseLines(input: string) {
  return input
    .split('\n')
    .map((line) => line.trim())
    .filter(Boolean);
}

function hexToBytes(input: string) {
  const tokens = input
    .replace(/,/g, ' ')
    .split(/\s+/)
    .map((item) => item.trim())
    .filter(Boolean);
  return tokens.map((token) => parseInt(token, 16)).filter((num) => !Number.isNaN(num) && num >= 0 && num <= 255);
}

async function withAction<T>(name: string, fn: () => Promise<T>) {
  const start = performance.now();
  try {
    const data = await fn();
    const cost = (performance.now() - start).toFixed(2);
    appendLog(`${name} success (${cost} ms)`, data);
    setOutput(data);
    return data;
  } catch (error) {
    const cost = (performance.now() - start).toFixed(2);
    appendLog(`${name} failed (${cost} ms)`, error);
    setOutput(error);
    throw error;
  }
}

async function apiStartWatch() {
  await withAction('startWatch', async () => {
    await startWatch();
    if (!unlistenClipboard) {
      unlistenClipboard = await onClipboardChange(() => {
        appendLog('onClipboardChange event');
      });
    }
    watchStarted.value = true;
    return { started: true };
  });
}

async function apiStopWatch() {
  await withAction('stopWatch', async () => {
    await stopWatch();
    if (unlistenClipboard) {
      await unlistenClipboard();
      unlistenClipboard = null;
    }
    watchStarted.value = false;
    return { stopped: true };
  });
}

async function apiAvailableFormats() {
  await withAction('availableFormats', availableFormats);
}

async function apiHasText() {
  await withAction('hasText', hasText);
}

async function apiHasRtf() {
  await withAction('hasRtf', hasRtf);
}

async function apiHasHtml() {
  await withAction('hasHtml', hasHtml);
}

async function apiHasImage() {
  await withAction('hasImage', hasImage);
}

async function apiHasFiles() {
  await withAction('hasFiles', hasFiles);
}

async function apiHasFormat() {
  await withAction('hasFormat', () => hasFormat(form.hasFormatName));
}

async function apiReadText() {
  await withAction('readText', readText);
}

async function apiReadRtf() {
  await withAction('readRtf', readRtf);
}

async function apiReadHtml() {
  await withAction('readHtml', readHtml);
}

async function apiReadImage() {
  const options: ReadImageOptions = {
    includeBytes: form.readImageIncludeBytes,
    autoSave: form.readImageAutoSave,
    preferRawPng: form.readImagePreferRaw,
  };
  await withAction('readImage', () => readImage(options));
}

async function apiReadFiles() {
  await withAction('readFiles', readFiles);
}

async function apiReadBuffer() {
  await withAction('readBuffer', () => readBuffer(form.readBufferFormat));
}

async function apiReadClipboard() {
  await withAction('readClipboard', () =>
    readClipboard({
      includeBytes: form.readImageIncludeBytes,
      autoSave: form.readImageAutoSave,
      preferRawPng: form.readImagePreferRaw,
    }),
  );
}

async function apiWriteText() {
  await withAction('writeText', () => writeText(form.writeTextValue));
}

async function apiWriteRtf() {
  await withAction('writeRtf', () => writeRtf(form.writeRtfValue));
}

async function apiWriteHtml() {
  await withAction('writeHtml', () => writeHtml(form.writeHtmlValue));
}

async function apiWriteImageFromPath() {
  await withAction('writeImage(path)', () =>
    writeImage({
      path: form.writeImagePath,
      preferRawPng: true,
      alsoSetStandardImage: form.writeImageAlsoStandard,
    }),
  );
}

async function apiWriteImageFromFile(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) {
    return;
  }
  const buffer = await file.arrayBuffer();
  const bytes = Array.from(new Uint8Array(buffer));
  await withAction('writeImageBytes(file)', () =>
    writeImageBytes({
      bytes,
      format: file.type || 'image/png',
      fastOnly: form.imageBytesFastOnly,
    }),
  );
  input.value = '';
}

async function apiWriteFiles() {
  const filesPath = parseLines(form.writeFilesText);
  await withAction('writeFiles', () => writeFiles(filesPath));
}

async function apiWriteVideoFiles() {
  const filesPath = parseLines(form.writeVideoFilesText);
  await withAction('writeVideoFiles', () => writeVideoFiles(filesPath));
}

async function apiWriteBuffer() {
  const data = hexToBytes(form.writeBufferDataHex);
  await withAction('writeBuffer', () => writeBuffer({ format: form.writeBufferFormat, data }));
}

async function apiClear() {
  await withAction('clear', clear);
}

async function apiGetFilePath() {
  await withAction('getFilePath', getFilePath);
}

async function runSmokeTests() {
  await withAction('Scenario: text roundtrip', async () => {
    await writeText(`smoke-${Date.now()}`);
    const text = await readText();
    return { text };
  });

  await withAction('Scenario: html and rtf', async () => {
    await writeHtml('<h3>Smoke HTML</h3><p>paragraph</p>');
    const html = await readHtml();
    await writeRtf('{\\rtf1\\ansi smoke rtf}');
    const rtf = await readRtf();
    return { html, rtf };
  });

  await withAction('Scenario: custom buffer', async () => {
    await writeBuffer({ format: 'application/x-demo-bin', data: [0xde, 0xad, 0xbe, 0xef] });
    const payload = await readBuffer('application/x-demo-bin');
    return payload;
  });

  await withAction('Scenario: read snapshot', async () => {
    return readClipboard({ includeBytes: false, preferRawPng: true });
  });
}

onBeforeUnmount(async () => {
  if (watchStarted.value) {
    await apiStopWatch();
  }
});
</script>

<template>
  <main class="page">
    <header class="hero">
      <h1>Clipboard Pro API Tester</h1>
      <p>Vue3 + Tauri v2 real-world manual test panel for all plugin APIs.</p>
    </header>

    <section class="card">
      <h2>Watcher APIs</h2>
      <div class="row">
        <button @click="apiStartWatch" :disabled="watchStarted">startWatch</button>
        <button @click="apiStopWatch" :disabled="!watchStarted">stopWatch</button>
        <button @click="runSmokeTests">Run Smoke Scenarios</button>
      </div>
    </section>

    <section class="card">
      <h2>Has & Format APIs</h2>
      <div class="row">
        <button @click="apiAvailableFormats">availableFormats</button>
        <button @click="apiHasText">hasText</button>
        <button @click="apiHasRtf">hasRtf</button>
        <button @click="apiHasHtml">hasHtml</button>
        <button @click="apiHasImage">hasImage</button>
        <button @click="apiHasFiles">hasFiles</button>
      </div>
      <div class="row">
        <input v-model="form.hasFormatName" placeholder="format, e.g. image/png" />
        <button @click="apiHasFormat">hasFormat</button>
      </div>
    </section>

    <section class="card">
      <h2>Read APIs</h2>
      <div class="row">
        <button @click="apiReadText">readText</button>
        <button @click="apiReadRtf">readRtf</button>
        <button @click="apiReadHtml">readHtml</button>
        <button @click="apiReadFiles">readFiles</button>
      </div>
      <div class="row">
        <label>
          <input type="checkbox" v-model="form.readImageIncludeBytes" />
          readImage includeBytes
        </label>
        <label>
          <input type="checkbox" v-model="form.readImagePreferRaw" />
          readImage preferRawPng
        </label>
        <label>
          <input type="checkbox" v-model="form.readImageAutoSave" />
          readImage autoSave (slower)
        </label>
        <button @click="apiReadImage">readImage</button>
      </div>
      <div class="row">
        <input v-model="form.readBufferFormat" placeholder="read buffer format" />
        <button @click="apiReadBuffer">readBuffer</button>
      </div>
      <div class="row">
        <button @click="apiReadClipboard">readClipboard</button>
      </div>
    </section>

    <section class="card">
      <h2>Write APIs</h2>
      <div class="grid-two">
        <div>
          <label>writeText</label>
          <textarea v-model="form.writeTextValue" rows="2" />
          <button @click="apiWriteText">writeText</button>
        </div>
        <div>
          <label>writeRtf</label>
          <textarea v-model="form.writeRtfValue" rows="2" />
          <button @click="apiWriteRtf">writeRtf</button>
        </div>
        <div>
          <label>writeHtml</label>
          <textarea v-model="form.writeHtmlValue" rows="3" />
          <button @click="apiWriteHtml">writeHtml</button>
        </div>
        <div>
          <label>writeImage(path)</label>
          <input v-model="form.writeImagePath" placeholder="absolute path to image file" />
          <button @click="apiWriteImageFromPath">writeImage(path)</button>
          <label class="inline">
            <input type="checkbox" v-model="form.writeImageAlsoStandard" />
            alsoSetStandardImage (compatibility, slower)
          </label>
          <label class="inline">
            <input type="checkbox" v-model="form.imageBytesFastOnly" />
            writeImageBytes fastOnly
          </label>
          <input type="file" accept="image/*" @change="apiWriteImageFromFile" />
        </div>
      </div>
    </section>

    <section class="card">
      <h2>File / Video / Buffer APIs</h2>
      <div class="grid-two">
        <div>
          <label>writeFiles (one path per line)</label>
          <textarea v-model="form.writeFilesText" rows="4" />
          <button @click="apiWriteFiles">writeFiles</button>
        </div>
        <div>
          <label>writeVideoFiles (one path per line)</label>
          <textarea v-model="form.writeVideoFilesText" rows="4" />
          <button @click="apiWriteVideoFiles">writeVideoFiles</button>
        </div>
        <div>
          <label>writeBuffer format</label>
          <input v-model="form.writeBufferFormat" />
          <label>writeBuffer hex bytes</label>
          <input v-model="form.writeBufferDataHex" placeholder="01 02 03 04" />
          <button @click="apiWriteBuffer">writeBuffer</button>
        </div>
      </div>
    </section>

    <section class="card">
      <h2>Maintenance APIs</h2>
      <div class="row">
        <button @click="apiGetFilePath">getFilePath</button>
        <button @click="apiClear">clear</button>
      </div>
    </section>

    <section class="card result">
      <h2>Latest Output</h2>
      <pre>{{ output }}</pre>
    </section>

    <section class="card logs">
      <h2>Action Logs</h2>
      <pre v-for="(item, idx) in logs" :key="idx">{{ item }}</pre>
    </section>
  </main>
</template>
