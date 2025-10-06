import { createApp } from 'vue';
import './styles.css';

import MixerPanel from './MixerPanel.vue';
import Overlay from './Overlay.vue';
import Settings from './Settings.vue';

const getWindowType = (): 'mixer' | 'overlay' | 'settings' => {
  const pathname = window.location.pathname;
  if (pathname.includes('mixer')) return 'mixer';
  if (pathname.includes('overlay')) return 'overlay';
  if (pathname.includes('settings')) return 'settings';

  throw new Error('Invalid window type');
};

const windowType = getWindowType();

let app;

switch (windowType) {
  case 'mixer':
    app = createApp(MixerPanel);
    break;
  case 'overlay':
    app = createApp(Overlay);
    break;
  case 'settings':
    app = createApp(Settings);
    break;
  default:
    throw new Error('Invalid window type');
}

app.mount('#root');
