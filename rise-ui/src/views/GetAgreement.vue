<script>
import { ethers } from 'ethers'
import contractABI from '@/assets/AgreementContract.json'
import { WalletLocalStorageKey, WalletAccountsKey, WalletPartnersKey } from '@/constants/index.js'
import { ReadWallet } from '../utils'

export default {
  data() {
    return {
      wallet: null,
      selectedAccount: '',
      selectedPartner: '',
      stationAddress: '',
      entityAddress: '',
      error: '',
      amount: '0 ETH',
      status: 'EMPTY',
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
      this.stationAddress = account.address
    },
    selectedPartner(name) {
      const partner = this.wallet.get(WalletPartnersKey).get(name)
      this.entityAddress = partner.address
    },
    stationAddress(address) {
      if (ethers.isAddress(address)) {
        this.clearAlerts()
      } else {
        this.error = 'Failed to parse station address.'
      }
    },
    entityAddress(address) {
      if (ethers.isAddress(address)) {
        this.clearAlerts()
      } else {
        this.error = 'Failed to parse entity address.'
      }
    },
  },
  created() {
    this.loadWallet()
  },
  methods: {
    clearAlerts() {
      this.error = ''
    },
    getAgreement() {
      const contractAddress = import.meta.env.VITE_AGREEMENT_CONTRACT_ADDRESS
      const provider = new ethers.getDefaultProvider(import.meta.env.VITE_RPC_URL)
      const contract = new ethers.Contract(contractAddress, contractABI.abi, provider)
      contract
        .get(this.stationAddress, this.entityAddress)
        .then((res) => {
          this.amount = `${ethers.formatEther(res[0])} ETH`
          let status = ''
          switch (res[1]) {
            case 0n: {
              status = 'EMPTY'
              break
            }
            case 1n: {
              status = 'CREATED'
              break
            }
            case 2n: {
              status = 'SIGNED'
              break
            }
          }
          this.status = status
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
  <h1>Get Agreement</h1>
  <div>
    <p>Amount: {{ amount }}</p>
    <p>Status: {{ status }}</p>
  </div>
  <div>
    <label for="stationAddress">Station address</label>
    <select id="stationAddress" v-model="selectedAccount">
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
    <button type="button" @click="getAgreement">Get</button>
  </div>
  <div>
    <p v-if="error !== ''" class="error alert">
      {{ error }}
    </p>
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
