import { createRouter, createWebHistory } from 'vue-router'
import CreateAgreement from '@/views/CreateAgreement.vue'
import SignAgreement from '@/views/SignAgreement.vue'
import GetAgreement from '@/views/GetAgreement.vue'
import WalletManagement from '@/views/WalletManagement.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: { name: 'walletManagement' },
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
    {
      path: '/get-agreement',
      name: 'getAgreement',
      component: GetAgreement,
    },
    {
      path: '/wallet',
      name: 'walletManagement',
      component: WalletManagement,
    },
  ],
})

export default router
