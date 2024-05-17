import { createRouter, createWebHistory } from 'vue-router'

import StreamSnapshotView from '@/views/StreamSnapshotView.vue'
import PaymentToolsView from '@/views/PaymentToolsView.vue'
import HomeView from '@/views/Home.vue'
import ImageUploadView from '@/views/ImageUploadView.vue'
import VideoUploadView from '@/views/VideoUploadView.vue'

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
        {
            path: '/photo-upload',
            name: 'photo-upload',
            component: ImageUploadView,
        },
        {
            path: '/video-upload',
            name: 'video-upload',
            component: VideoUploadView,
        },
    ],
})

export default router
