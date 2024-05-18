<script setup>
import NexaPartners from '@/components/NexaPartners.vue'
import NexaSponsors from '@/components/NexaSponsors.vue'
import Picture from '@/components/Picture.vue'
</script>

<template>
    <!--
    <h1>Web3 Drone Corridor</h1>
    <NexaPartners />
    -->
    <div class="container">
        <div>
            <div class="desktop">
                <Picture src="/images/nexa-banner-desktop.png" alt="NEXA." dynamic />
            </div>
            <div class="mobile">
                <Picture src="/images/nexa-banner-mobile.png" alt="NEXA." dynamic />
            </div>
            <div class="button-container">
                <a href="/payment" class="button-link-large">Get my video!</a>
            </div>
        </div>
    </div>
    <!--
    <NexaSponsors />
    -->
</template>

<style scoped>
.button-link-large {
    margin-top: 2rem;
    margin-bottom: 2rem;
}

.container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: calc(100vh - 2 * 4px);
}

.container > div {
    display: inline-block;
}

:deep(img) {
    width: 100vw !important;
    max-width: 100vw !important;
}

@media (orientation: landscape) and (max-width: 40rem) {
    :deep(img) {
        max-height: 65vh !important;
    }
}

@media (min-width: 40rem) {
    .desktop {
        display: block;
    }
    .mobile {
        display: none;
    }
}

@media (max-width: 40rem) {
    .desktop {
        display: none;
    }
    .mobile {
        display: block;
    }
}
</style>
