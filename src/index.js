import initUno from '@unocss/runtime';
import preset from '@unocss/preset-mini';

import { initCore } from './init_core';
import './style/style.less';

initUno({
  defaults: { presets: [preset()] },
});

initCore();
