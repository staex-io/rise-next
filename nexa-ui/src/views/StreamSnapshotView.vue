<script setup>
import Picture from '@/components/Picture.vue'
import PhotoLicense from '@/components/PhotoLicense.vue'
import sha256 from 'js-sha256'
import { Base64 } from 'js-base64'
import { ethers } from 'ethers'
import contractABI from '@/assets/DataProvingContract.json'
</script>
<script>
export default {
    data() {
        return {
            timeoutId: null,
            playId: null,
            rawImage: null,
            videoSrc: '',
            hash: '',
        }
    },
    computed: {
        snapshot() {
            if (this.rawImage === null) return
            const snapshot = URL.createObjectURL(this.rawImage)
            return snapshot
        },
    },
    created() {
        this.timeoutId = setTimeout(this.getSnapshot, 1)
    },
    methods: {
        async getSnapshot() {
            try {
                const res = await fetch('/video.mp4')
                if (res.status === 404) {
                    throw 'not found'
                }
                const rawImage = await res.blob()

                const rawImageByteArray = new Uint8Array(await rawImage.arrayBuffer())
                console.log(rawImageByteArray.length)
                const hash = sha256.create()
                hash.update(rawImageByteArray)
                hash.finalize()

                const encoded = Base64.fromUint8Array(new Uint8Array(hash.array()))

                /*
        const contractAddress = import.meta.env.VITE_DATA_PROVING_CONTRACT_ADDRESS
        const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
        const contract = new ethers.Contract(contractAddress, contractABI.abi, provider)
        const expected = await contract.get()

        if (encoded != expected) throw 'hash mismatch'
        */

                clearTimeout(this.timeoutId)
                this.rawImage = rawImage
                this.hash = encoded
                this.videoSrc = '/video.mp4'
                //this.playId = setTimeout(this.playVideo, 1000)
                return
            } catch (e) {
                console.error(e)
            }
            this.timeoutId = setTimeout(this.getSnapshot, 1000)
        },
        /*
        playVideo() {
            const video = document.getElementById('video')
            if (video) {
                video.play()
                clearTimeout(this.playId)
                this.playId = null
            } else {
                this.playId = setTimeout(this.playVideo, 1000)
            }
        },
        */
        async downloadImage() {
            if (!this.rawImage) return
            const imageUrl = URL.createObjectURL(
                new Blob([new Uint8Array(await this.rawImage.arrayBuffer()).buffer], {
                    type: 'video/mp4',
                }),
            )
            const link = document.createElement('a')
            link.href = imageUrl
            link.download = `nexa-video.mp4`
            document.body.appendChild(link)
            link.dispatchEvent(
                new MouseEvent('click', {
                    bubbles: true,
                    cancelable: true,
                    view: window,
                }),
            )
            document.body.removeChild(link)
        },
    },
}
</script>

<template>
    <div v-if="!rawImage">
        <h1 class="center">Please wait…</h1>
    </div>

    <div v-else>
        <h1 class="center">Enjoy your video!</h1>
        <div class="margin">
            <video id="video" controls loop autoplay playsinline muted preload="metadata">
                <source type="video/mp4" :src="videoSrc" />
            </video>
            <!--
      <Picture
        :src="snapshot"
        alt="If you do not see the image, please download it using the button."
        preset="x400"
        dynamic
      />
        -->
            <PhotoLicense />
        </div>
    </div>
    <div class="button-container">
        <button @click="downloadImage" v-if="rawImage" class="large">Download</button>
        <!--
    <a v-if="rawImage" href="/video.mp4" class="button-link-large">Download</a>
      -->
    </div>
    <div class="button-container">
        <span v-if="!rawImage" class="loader" />
    </div>
    <div v-if="rawImage" class="margin">
        <p>
            Video was validated using smart contract on Lisk L2 network. You can check the hash in
            <a
                href="https://sepolia-blockscout.lisk.com/address/0x0BD357DB61671f31fF0A75eb403C13E628C9242e"
                target="_blank"
                >block explorer</a
            >.
        </p>
    </div>
    <div v-if="rawImage" class="check-container">
        <img src="/check.svg" width="50px" /> <small class="hash">{{ hash }}</small>
    </div>
</template>

<style scoped>
.margin {
    margin-top: 1rem;
}
.hash {
    word-break: break-all;
    max-width: 14rem;
    display: inline-block;
}
.center {
    text-align: center;
}
.check-container {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
}

.check-container > * {
    margin: 1rem;
}
</style>
