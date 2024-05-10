<script>
import { WalletLocalStorageKey, WalletAccountsKey } from '@/constants/index.js'
import { ReadWallet } from '../utils'
import { ethers } from 'ethers'
import contractABI from '@/assets/DIDContract.json'

export default {
  data() {
    return {
      wallet: null,
      selectedAccount: '',
      selectedAccountAddress: '',
      selectedAccountPrivateKey: '',
      station: null,
      location: '',
      price: '',
      error: '',
      success: '',
    }
  },
  computed: {
    accountsAsArr() {
      return Array.from(this.wallet.get(WalletAccountsKey).entries())
    },
  },
  watch: {
    selectedAccount(name) {
      this.clearAlerts()
      const account = this.wallet.get(WalletAccountsKey).get(name)
      this.selectedAccountAddress = account.address
      this.selectedAccountPrivateKey = account.privateKey
      this.load()
    },
  },
  created() {
    this.loadWallet()
  },
  methods: {
    clearAlerts() {
      this.error = ''
      this.success = ''
    },
    load() {
      this.station = null
      this.location = ''
      this.price = ''
      const contractAddress = import.meta.env.VITE_DID_CONTRACT_ADDRESS
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const signer = new ethers.Wallet(this.selectedAccountPrivateKey, provider)
      const contract = new ethers.Contract(contractAddress, contractABI.abi, signer)
      contract
        .get(this.selectedAccountAddress)
        .then((res) => {
          if (res[0] !== '') {
            this.station = {
              address: this.selectedAccountAddress,
              location: res[0],
              price: ethers.formatEther(res[1]),
            }
            this.location = res[0]
            this.price = ethers.formatEther(res[1])
          }
        })
        .catch((error) => {
          this.error = error
        })
    },
    update() {
      if (this.location === '' || this.price === '') return
      const contractAddress = import.meta.env.VITE_DID_CONTRACT_ADDRESS
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const signer = new ethers.Wallet(this.selectedAccountPrivateKey, provider)
      const contract = new ethers.Contract(contractAddress, contractABI.abi, signer)
      contract
        .update(this.location, ethers.parseEther(this.price.toString()))
        .then(() => {
          this.success = 'Successfully updated'
        })
        .catch((error) => {
          this.error = error
        })
    },
    loadWallet() {
      const walletJSON = localStorage.getItem(WalletLocalStorageKey)
      const wallet = ReadWallet(walletJSON)
      this.wallet = wallet
    },
  },
}
</script>

<template>
  <h1>Decentralized identity</h1>
  <div class="local-row">
    <select v-model="selectedAccount">
      <option disabled value="" selected>Select an account</option>
      <option v-for="[key, value] in accountsAsArr" :key="key" :value="key">
        {{ value.name }}
      </option>
    </select>
  </div>
  <div class="local-row">
    <p v-if="error !== ''" class="error alert">
      {{ error }}
    </p>
  </div>
  <div class="local-row">
    <p v-if="success !== ''" class="success alert">
      {{ success }}
    </p>
  </div>
  <div v-if="station" class="card local-row">
    <div class="card-header">Station</div>
    <div class="card-content">
      <div class="card-field">
        <span class="card-field-label">Address</span>
        <span class="card-field-value">
          <div class="h-scroll-container">{{ station.address }}</div></span
        >
      </div>
      <div class="card-field">
        <span class="card-field-label">Location</span>
        <span class="card-field-value">
          <div class="h-scroll-container">
            <a :href="`https://www.google.com/maps/place/${station.location}`" target="_blank">
              {{ station.location }}
            </a>
          </div>
        </span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Price</span>
        <span class="card-field-value">{{ station.price }} ETH</span>
      </div>
    </div>
  </div>
  <div class="local-row">
    <label for="location">Location</label>
    <input id="location" v-model="location" type="text" name="location" />
  </div>
  <div class="local-row">
    <label for="price">Price in ethers (ex: to settle 0.01 ether write just 0.01)</label>
    <input id="price" v-model="price" type="number" name="price" />
  </div>
  <div class="local-row">
    <button type="button" @click="update">Update</button>
  </div>
</template>

<style scoped>
.local-row {
  width: 75%;
  margin: 0 auto 25px auto;
}

button {
  width: 100%;
  margin: auto;
}

.alert {
  margin-top: 25px;
}
</style>
