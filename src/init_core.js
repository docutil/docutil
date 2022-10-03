import init, { initApp, Config } from '@docutil/core';

let INITED = false;

export async function initCore() {
  if (INITED) {
    return;
  }

  await init();
  const { title, root, footer, searchApiEndpoint } = window.config || {};
  const config = new Config()
    .setFooterMessage(footer || '')
    .setRootPath(root || '/')
    .setTitle(title || 'docutil')
    .setSearchApiEndpoint(searchApiEndpoint || '');

  initApp(config);
  INITED = true;
}
