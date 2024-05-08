<script>
import { ethers } from 'ethers'
import QRCode from 'qrcode'

export default {
  data() {
    return {
      address: '',
      qrcode: '',
      error: '',
    }
  },
  watch: {
    address(value) {
      this.address = value.trim().toLowerCase()
      this.generateQrCode()
    },
  },
  methods: {
    clearAlerts() {
      this.error = ''
    },
    generateQrCode() {
      this.clearAlerts()
      if (this.address === '') {
        this.qrcode = ''
        return
      }
      if (!ethers.isAddress(this.address)) {
        this.error = 'Failed to parse address.'
        return
      }
      const options = {
        errorCorrectionLevel: 'H',
        quality: 1,
      }
      let object = {
        address: this.address,
      }
      QRCode.toDataURL(JSON.stringify(object), options).then((url) => {
        this.qrcode = url
      })
    },
  },
}
</script>

<template>
  <h1>Get your QR code</h1>
  <div>
    <p v-if="error !== ''" class="error alert">
      {{ error }}
    </p>
  </div>
  <div>
    <label for="address">Address</label>
    <input id="address" v-model="address" type="text" name="address" />
  </div>
  <div v-if="qrcode !== ''">
    <br />
    <h2>QR code</h2>
    <img :src="qrcode" alt="QR code" class="qrcode" />
    <ul>
      <li>Print this code and stick it to the bottom of your drone.</li>
      <li>
        The airport will have similar code but with their wallet address on top of each landing pad.
      </li>
      <li>When your drone lands send the money to the airport using this code.</li>
    </ul>
  </div>
</template>

<style scoped>
.qrcode {
  display: block;
  margin-bottom: 20px;
}
</style>
