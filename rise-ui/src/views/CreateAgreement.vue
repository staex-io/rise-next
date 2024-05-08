<script>
import { ethers } from 'ethers'
import contractABI from '@/assets/AgreementContract.json'
import { WalletLocalStorageKey, WalletAccountsKey, WalletPartnersKey } from '@/constants/index.js'
import { ReadWallet } from '@/utils/index.js'

export default {
  data() {
    return {
      wallet: null,
      selectedAccount: '',
      selectedPartner: '',
      stationPrivateKey: '',
      entityAddress: '',
      amount: 10,
      error: '',
      success: '',
    }
  },
  computed: {
    accountsAsArr() {
      return Array.from(this.wallet.get(WalletAccountsKey).entries())
    },
    partnersAsArr() {
      return Array.from(this.wallet.get(WalletPartnersKey).entries())
    },
  },
  watch: {
    selectedAccount(name) {
      const account = this.wallet.get(WalletAccountsKey).get(name)
      this.stationPrivateKey = account.privateKey
    },
    selectedPartner(name) {
      const partner = this.wallet.get(WalletPartnersKey).get(name)
      this.entityAddress = partner.address
    },
    stationPrivateKey(privateKey) {
      try {
        new ethers.Wallet(privateKey)
        this.clearAlerts()
      } catch (error) {
        this.error = 'Failed to parse station private key.'
      }
    },
    entityAddress(address) {
      if (ethers.isAddress(address)) {
        this.clearAlerts()
      } else {
        this.error = 'Failed to parse entity address.'
      }
    },
    amount(amount) {
      const num = parseFloat(amount)
      if (isNaN(num)) {
        this.error = 'Amount is not a number.'
      } else {
        this.clearAlerts()
      }
    },
  },
  methods: {
    clearAlerts() {
      this.error = ''
      this.success = ''
    },
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
    loadWallet() {
      const walletJSON = localStorage.getItem(WalletLocalStorageKey)
      const wallet = ReadWallet(walletJSON)
      this.wallet = wallet
    },
  },
  created() {
    this.loadWallet()
  },
}
</script>

<template>
  <h1>Create Agreement</h1>
  <div>
    <label for="stationPrivateKey">Station private key</label>
    <select id="stationPrivateKey" v-model="selectedAccount">
      <option disabled value="" selected>Select an account</option>
      <option v-for="[key, value] in accountsAsArr" :key="key" :value="key">
        {{ value.name }}
      </option>
    </select>
  </div>
  <div>
    <label for="entityAddress">Entity address</label>
    <select id="entityAddress" v-model="selectedPartner">
      <option disabled value="" selected>Select a partner</option>
      <option v-for="[key, value] in partnersAsArr" :key="key" :value="key">
        {{ value.name }}
      </option>
    </select>
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
