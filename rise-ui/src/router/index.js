import { createRouter, createWebHistory } from 'vue-router'

import CreateAgreement from '@/views/CreateAgreement.vue'
import SignAgreement from '@/views/SignAgreement.vue'
import GetAgreement from '@/views/GetAgreement.vue'
import WalletManagement from '@/views/WalletManagement.vue'
import DIDManagement from '@/views/DIDManagement.vue'
import QRCode from '@/views/QRCode.vue'
import AgreementsView from '@/views/AgreementsView.vue'
import LandingsView from '@/views/LandingsView.vue'
import LandingView from '@/views/LandingView.vue'
import StatsView from '@/views/StatsView.vue'
import StationsView from '@/views/StationsView.vue'
import StreamSnapshotView from '@/views/StreamSnapshotView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: { name: 'landings' },
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
      path: '/agreements',
      name: 'Agreements',
      component: AgreementsView,
    },
    {
      path: '/wallet',
      name: 'walletManagement',
      component: WalletManagement,
    },
    {
      path: '/did',
      name: 'didManagement',
      component: DIDManagement,
    },
    {
      path: '/stations',
      name: 'stations',
      component: StationsView,
    },
    {
      path: '/landing/:id',
      name: 'landing',
      component: LandingView,
    },
    {
      path: '/landings',
      name: 'landings',
      component: LandingsView,
    },
    {
      path: '/stats',
      name: 'stats',
      component: StatsView,
    },
    {
      path: '/qrcode',
      name: 'QRCode',
      component: QRCode,
    },
    {
      path: '/stream/snapshot',
      name: 'streamSnapshot',
      component: StreamSnapshotView,
    },
  ],
})

export default router
