<script setup>
import NexaPartners from '@/components/NexaPartners.vue'
import NexaSponsors from '@/components/NexaSponsors.vue'
import Picture from '@/components/Picture.vue'
</script>

<template>
    <!--
    <h1>Web3 Drone Corridor</h1>
    -->
    <NexaPartners />
    <!--
    <Picture src="/images/banner-nexa.svg" alt="NEXA." dynamic />
    -->
    <div class="button-container">
        <a href="/payment" class="button-link-large">Get my video!</a>
    </div>
    <NexaSponsors />
</template>

<style scoped>
.button-link-large {
    margin-top: 2rem;
    margin-bottom: 2rem;
}
</style>
