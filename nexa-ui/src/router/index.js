import { createRouter, createWebHistory } from 'vue-router'

import StreamSnapshotView from '@/views/StreamSnapshotView.vue'
import PaymentToolsView from '@/views/PaymentToolsView.vue'
import HomeView from '@/views/Home.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/payment',
      name: 'payment',
      component: PaymentToolsView,
    },
    {
      path: '/photo',
      name: 'photo',
      component: StreamSnapshotView,
    },
  ],
})

export default router
