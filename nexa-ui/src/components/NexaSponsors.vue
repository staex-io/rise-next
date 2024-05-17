<script setup>
import Picture from '@/components/Picture.vue'
</script>

<template>
    <ul class="partners">
        <li class="im-marienpark-bg">
            <a href="https://marienpark.berlin/">
                <Picture
                    src="/images/im-marienpark-logo.png"
                    alt="Im Marienpark logo."
                    preset="128x128" />
            </a>
        </li>
        <li>
            <a href="https://www.thedrivery.com/">
                <Picture
                    src="/images/drivery.png"
                    alt="The Drivery logo."
                    preset="128x128" />
            </a>
        </li>
        <li id="icp">
            <a href="https://internetcomputer.org/">
                <Picture
                    src="/images/icp-logo.png"
                    alt="ICP logo."
                    preset="128x128" />
            </a>
        </li>
    </ul>
</template>

<style scoped>
ul.partners, ul.sponsors {
    list-style-type: none;
    padding: 0;
    margin-top: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
}

ul.partners > li, ul.sponsors > li {
    display: inline-block;
}

.partners :deep(img) {
    max-width: 160px;
    max-height: 160px;
    filter: grayscale(1);
}

.sponsors :deep(img) {
    max-width: 128px;
    max-height: 128px;
    filter: grayscale(1);
}

#icp :deep(img) {
    max-width: 96px;
    max-height: 96px;
}

.partners li, .sponsors li {
    display: inline-block;
    vertical-align: middle;
    margin: 0.25rem;
    padding: 0.25rem;
}

h2 {
    margin-top: 2rem !important;
}

.im-marienpark-bg {
    background: rgb(0, 0, 0);
}

.icp-bg {
    background: #3b00b9;
}
</style>

