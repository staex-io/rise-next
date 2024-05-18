<script setup>
import { RouterLink, RouterView } from 'vue-router'
import VueCookieAcceptDecline from 'vue-cookie-accept-decline'
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
    <!--
    <footer>
        <VueCookieAcceptDecline
            :debug="false"
            :disableDecline="true"
            :showPostponeButton="false"
            @clicked-accept="cookieClickedAccept"
            @clicked-decline="cookieClickedDecline"
            @clicked-postpone="cookieClickedPostpone"
            @removed-cookie="cookieRemovedCookie"
            @status="cookieStatus"
            elementId="myPanel1"
            position="bottom-left"
            ref="myPanel1"
            transitionName="slideFromBottom"
            type="floating"
        >
            <template #postponeContent>&times;</template>
            <template #message><span></span></template>
            <template #declineContent>Opt Out</template>
            <template #acceptContent>We use cookies to improve user experience. <button>OK</button></template>
        </VueCookieAcceptDecline>
    </footer>
    -->
</template>
