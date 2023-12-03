import { createRouter, createWebHistory } from 'vue-router'
import CreateAgreement from '@/views/CreateAgreement.vue'
import SignAgreement from '@/views/SignAgreement.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: { name: 'createAgreement' },
    },
    {
      path: '/create-agreement',
      name: 'createAgreement',
      component: CreateAgreement,
    },
    {
      path: '/sign-agreement',
      name: 'signAgreement',
      component: SignAgreement,
    },
  ],
})

export default router
