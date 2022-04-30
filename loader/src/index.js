import initUnocss from '@unocss/runtime';
import preset from '@unocss/preset-mini';

import init, { initApp, Config } from '../../core/pkg';
import './style.css';

!(async () => {
  await init();

  const { title, root, footer } = window.config || {};
  const config = new Config()
    .setFooterMessage(footer || '')
    .setRootPath(root || '/')
    .setTitle(title || 'docutil');

  initApp(config);

  initUnocss({
    defaults: { presets: [preset()] },
  });
})();
