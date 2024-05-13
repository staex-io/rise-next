<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>
<script>
import router from '@/router'
export default {
    beforeMount() {
        router.push({ path: window.location.pathname })
    },
}
</script>

<template>
    <main>
        <RouterView />
    </main>
</template>
