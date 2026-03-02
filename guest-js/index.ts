import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const buildCmd = (cmd: string) => `plugin:clipboard-pro|${cmd}`;
const buildEvent = (event: string) => `plugin:clipboard-pro://${event}`;

export const COMMANDS = {
  START_WATCH: buildCmd('start_watch'),
  STOP_WATCH: buildCmd('stop_watch'),
  AVAILABLE_FORMATS: buildCmd('available_formats'),
  HAS_TEXT: buildCmd('has_text'),
  HAS_RTF: buildCmd('has_rtf'),
  HAS_HTML: buildCmd('has_html'),
  HAS_IMAGE: buildCmd('has_image'),
  HAS_FILES: buildCmd('has_files'),
  HAS_FORMAT: buildCmd('has_format'),
  READ_TEXT: buildCmd('read_text'),
  READ_RTF: buildCmd('read_rtf'),
  READ_HTML: buildCmd('read_html'),
  READ_IMAGE: buildCmd('read_image'),
  READ_FILES: buildCmd('read_files'),
  READ_BUFFER: buildCmd('read_buffer'),
  READ_CLIPBOARD: buildCmd('read_clipboard'),
  WRITE_TEXT: buildCmd('write_text'),
  WRITE_RTF: buildCmd('write_rtf'),
  WRITE_HTML: buildCmd('write_html'),
  WRITE_IMAGE: buildCmd('write_image'),
  WRITE_IMAGE_BYTES: buildCmd('write_image_bytes'),
  WRITE_FILES: buildCmd('write_files'),
  WRITE_VIDEO_FILES: buildCmd('write_video_files'),
  WRITE_BUFFER: buildCmd('write_buffer'),
  CLEAR: buildCmd('clear'),
  GET_FILE_PATH: buildCmd('get_file_path'),
} as const;

export const EVENTS = {
  CLIPBOARD_CHANGE: buildEvent('clipboard_change'),
} as const;

export interface ReadImageOptions {
  includeBytes?: boolean;
  saveTo?: string;
  autoSave?: boolean;
  preferRawPng?: boolean;
}

export interface ReadImage {
  path?: string;
  width: number;
  height: number;
  size: number;
  format: string;
  bytes?: number[];
}

export type FileKind = 'image' | 'video' | 'directory' | 'file' | 'unknown';

export interface FileItem {
  path: string;
  size: number;
  isDir: boolean;
  kind: FileKind;
  mime?: string;
}

export interface ReadFiles {
  files: FileItem[];
  size: number;
}

export interface BufferPayload {
  format: string;
  data: number[];
}

export interface WriteImageRequest {
  path?: string;
  bytes?: number[];
  preferRawPng?: boolean;
  alsoSetStandardImage?: boolean;
}

export interface WriteImageBytesRequest {
  bytes: number[];
  format?: string;
  fastOnly?: boolean;
}

export interface ClipboardSnapshot {
  availableFormats: string[];
  text?: string;
  rtf?: string;
  html?: string;
  image?: ReadImage;
  files?: ReadFiles;
}

export const startWatch = () => invoke<void>(COMMANDS.START_WATCH);
export const stopWatch = () => invoke<void>(COMMANDS.STOP_WATCH);
export const availableFormats = () => invoke<string[]>(COMMANDS.AVAILABLE_FORMATS);
export const hasText = () => invoke<boolean>(COMMANDS.HAS_TEXT);
export const hasRtf = () => invoke<boolean>(COMMANDS.HAS_RTF);
export const hasHtml = () => invoke<boolean>(COMMANDS.HAS_HTML);
export const hasImage = () => invoke<boolean>(COMMANDS.HAS_IMAGE);
export const hasFiles = () => invoke<boolean>(COMMANDS.HAS_FILES);

export const hasFormat = (format: string) => invoke<boolean>(COMMANDS.HAS_FORMAT, { format });

export const readText = () => invoke<string>(COMMANDS.READ_TEXT);
export const readRtf = () => invoke<string>(COMMANDS.READ_RTF);
export const readHtml = () => invoke<string>(COMMANDS.READ_HTML);
export const readImage = (options?: ReadImageOptions) =>
  invoke<ReadImage>(COMMANDS.READ_IMAGE, { options });
export const readFiles = () => invoke<ReadFiles>(COMMANDS.READ_FILES);
export const readBuffer = (format: string) =>
  invoke<BufferPayload>(COMMANDS.READ_BUFFER, { format });
export const readClipboard = (options?: ReadImageOptions) =>
  invoke<ClipboardSnapshot>(COMMANDS.READ_CLIPBOARD, { options });

export const writeText = (content: string) => invoke<void>(COMMANDS.WRITE_TEXT, { content });
export const writeRtf = (content: string) => invoke<void>(COMMANDS.WRITE_RTF, { content });
export const writeHtml = (content: string) => invoke<void>(COMMANDS.WRITE_HTML, { content });
export const writeImage = (request: WriteImageRequest) =>
  invoke<void>(COMMANDS.WRITE_IMAGE, { request });
export const writeImageBytes = (request: WriteImageBytesRequest) =>
  invoke<void>(COMMANDS.WRITE_IMAGE_BYTES, { request });
export const writeFiles = (filesPath: string[]) =>
  invoke<void>(COMMANDS.WRITE_FILES, { filesPath });
export const writeVideoFiles = (filesPath: string[]) =>
  invoke<void>(COMMANDS.WRITE_VIDEO_FILES, { filesPath });
export const writeBuffer = (payload: BufferPayload) =>
  invoke<void>(COMMANDS.WRITE_BUFFER, { payload });

export const clear = () => invoke<void>(COMMANDS.CLEAR);
export const getFilePath = () => invoke<string>(COMMANDS.GET_FILE_PATH);

/**
 * EN: Listen to clipboard changes from the plugin watcher.
 * CN: 监听插件发出的剪贴板变化事件。
 */
export const onClipboardChange = async (callback: () => void) => {
  return listen(EVENTS.CLIPBOARD_CHANGE, () => callback());
};
