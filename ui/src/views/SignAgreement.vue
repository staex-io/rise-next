<script>
import { ethers } from 'ethers'
import contractABI from '@/assets/AgreementContract.json'

export default {
  data() {
    return {
      entityPrivateKey: import.meta.env.VITE_ENTITY_PRIVATE_KEY,
      stationAddress: import.meta.env.VITE_STATION_ADDRESS,
      amount: 10,
      error: '',
      success: '',
    }
  },
  watch: {
    entityPrivateKey(privateKey) {
      try {
        new ethers.Wallet(privateKey)
        this.error = ''
      } catch (error) {
        this.error = 'Failed to parse entity private key.'
      }
    },
    stationAddress(address) {
      if (ethers.isAddress(address)) {
        this.error = ''
      } else {
        this.error = 'Failed to parse station address.'
      }
    },
    amount(amount) {
      const num = parseFloat(amount)
      if (isNaN(num)) {
        this.error = 'Amount is not a number.'
      } else {
        this.error = ''
      }
    },
  },
  methods: {
    signAgreement() {
      if (
        this.error !== '' ||
        this.entityPrivateKey === '' ||
        this.stationAddress === '' ||
        this.amount == ''
      )
        return
      const contractAddress = import.meta.env.VITE_AGREEMENT_CONTRACT_ADDRESS
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const signer = new ethers.Wallet(this.entityPrivateKey, provider)
      const contract = new ethers.Contract(contractAddress, contractABI.abi, signer)
      contract
        .sign(this.stationAddress, ethers.parseEther(this.amount.toString()))
        .then(() => {
          this.success = 'Successfully signed'
        })
        .catch((error) => {
          this.error = error
        })
    },
  },
}
</script>

<template>
  <h1>Sign Agreement</h1>
  <div>
    <label for="entityPrivateKey">Entity private key</label>
    <input type="text" name="entityPrivateKey" id="entityPrivateKey" v-model="entityPrivateKey" />
  </div>
  <div>
    <label for="stationAddress">Station address</label>
    <input type="text" name="stationAddress" id="stationAddress" v-model="stationAddress" />
  </div>
  <div>
    <label for="amount">Amount in ethers (ex: to settle 0.01 ether write just 0.01)</label>
    <input type="number" name="amount" id="amount" v-model="amount" />
  </div>
  <div>
    <button type="button" @click="signAgreement">Sign</button>
  </div>
  <div>
    <p class="error alert" v-if="error !== ''">{{ error }}</p>
  </div>
  <div>
    <p class="success alert" v-if="success !== ''">{{ success }}</p>
  </div>
</template>

<style scoped>
div {
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
