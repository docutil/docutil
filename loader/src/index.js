import init, { main } from '../../core/pkg';
import './style.css';

!(async () => {
  await init();
  main();
})();
