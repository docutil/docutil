import initUno from '@unocss/runtime';
import preset from '@unocss/preset-mini';

import initWasm, { initApp, Config } from '../../core/pkg';
import './style.css';

initUno({
  defaults: { presets: [preset()] },
});

!(async () => {
  await initWasm();

  const { title, root, footer } = window.config || {};
  const config = new Config()
    .setFooterMessage(footer || '')
    .setRootPath(root || '/')
    .setTitle(title || 'docutil');

  initApp(config);
})();
