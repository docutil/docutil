import init, { main, Config } from '../../core/pkg';
import './style.css';

!(async () => {
  await init();

  const { title, root, footer } = window.config || {};
  const config = new Config()
    .set_footer_message(footer || '')
    .set_root_path(root || '/')
    .set_title(title || 'docutil');

  main(config);
})();
