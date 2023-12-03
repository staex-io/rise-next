<script>
import { ethers } from 'ethers'
import contractABI from '@/assets/AgreementContract.json'

export default {
  data() {
    return {
      stationPrivateKey: import.meta.env.VITE_STATION_PRIVATE_KEY,
      entityAddress: import.meta.env.VITE_ENTITY_ADDRESS,
      amount: 10,
      error: '',
      success: '',
    }
  },
  watch: {
    stationPrivateKey(privateKey) {
      try {
        new ethers.Wallet(privateKey)
        this.error = ''
      } catch (error) {
        this.error = 'Failed to parse station private key.'
      }
    },
    entityAddress(address) {
      if (ethers.isAddress(address)) {
        this.error = ''
      } else {
        this.error = 'Failed to parse entity address.'
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
    createAgreement() {
      if (
        this.error !== '' ||
        this.stationPrivateKey === '' ||
        this.entityAddress === '' ||
        this.amount == ''
      )
        return
      const contractAddress = import.meta.env.VITE_AGREEMENT_CONTRACT_ADDRESS
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const signer = new ethers.Wallet(this.stationPrivateKey, provider)
      const contract = new ethers.Contract(contractAddress, contractABI.abi, signer)
      contract
        .create(this.entityAddress, ethers.parseEther(this.amount.toString()))
        .then(() => {
          this.success = 'Successfully created.'
        })
        .catch((error) => {
          this.error = error
        })
    },
  },
}
</script>

<template>
  <h1>Create Agreement</h1>
  <div>
    <label for="stationPrivateKey">Station private key</label>
    <input
      type="text"
      name="stationPrivateKey"
      id="stationPrivateKey"
      v-model="stationPrivateKey"
    />
  </div>
  <div>
    <label for="entityAddress">Entity address</label>
    <input type="text" name="entityAddress" id="entityAddress" v-model="entityAddress" />
  </div>
  <div>
    <label for="amount">Amount in ethers (ex: to settle 0.01 ether write just 0.01)</label>
    <input type="number" name="amount" id="amount" v-model="amount" />
  </div>
  <div>
    <button type="button" @click="createAgreement">Create</button>
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
