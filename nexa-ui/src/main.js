import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import '@fontsource/roboto/300.css'
import '@fontsource/roboto/500.css'
import '@fontsource/roboto/700.css'
import '@fontsource/roboto-mono/300.css'
import '@fontsource/roboto-mono/500.css'
import './assets/main.css'
//import './assets/vue-cookie-accept-decline.css'

const app = createApp(App)
app.use(router)
app.mount('#app')
