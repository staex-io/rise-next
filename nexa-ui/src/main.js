import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import '@fontsource/roboto/300.css'
import '@fontsource/roboto/500.css'
import './assets/main.css'

const app = createApp(App)
app.use(router)
app.mount('#app')
