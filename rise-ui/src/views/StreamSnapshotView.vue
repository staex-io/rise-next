<script setup>
import sha256 from 'js-sha256'
import { Base64 } from 'js-base64'
</script>
<script>
export default {
  data() {
    return {
      timeoutId: null,
      rawImage: null,
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
        const res = await fetch('http://127.0.0.1:8090/stream_snapshot.jpg')
        this.rawImage = await res.blob()

        const rawImageByteArray = new Uint8Array(await this.rawImage.arrayBuffer())
        const hash = sha256.create()
        hash.update(rawImageByteArray)
        hash.finalize()

        const encoded = Base64.fromUint8Array(new Uint8Array(hash.array()))

        console.log(encoded)

        clearTimeout(this.timeoutId)
        return
      } catch (e) {
        console.error(e)
      }
      this.timeoutId = setTimeout(this.getSnapshot, 1000)
    },
    async downloadImage() {
      if (!this.rawImage) return
      const imageUrl = URL.createObjectURL(
        new Blob([new Uint8Array(await this.rawImage.arrayBuffer()).buffer], {
          type: 'image/jpeg',
        }),
      )
      const link = document.createElement('a')
      link.href = imageUrl
      link.download = `nexa.photo.jpeg`
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
    <h1>Please, wait for your photo. It is in process!</h1>
  </div>
  <div v-else>
    <h1>Enjoy your photo!</h1>

    <div>
      <img
        ref="image"
        :src="snapshot"
        alt="In case you do not see the image, you still should have a possibility to download it using the button"
      />
    </div>
  </div>
  <div style="margin-top: 25px">
    <button style="width: 25%" @click="downloadImage">
      <span v-if="rawImage">Download</span>
      <div v-else class="loader" />
    </button>
  </div>
</template>

<style scoped>
div {
  text-align: center;
}
</style>
