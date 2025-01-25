import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import FontAwesomeIcon from './plugins/fontawesome-icons';
import vue3GoogleLogin from 'vue3-google-login'

const googleClientId = import.meta.env.VITE_GOOGLE_CLIENT_ID;

createApp(App)
    .component('font-awesome-icon', FontAwesomeIcon)
    .use(vue3GoogleLogin, { clientId: googleClientId })
    .mount('#app')
