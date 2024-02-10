<script>
import { WalletLocalStorageKey, WalletAccountsKey } from '@/constants/index.js'
import { ReadWallet } from '../utils'

export default {
  data() {
    return {
      wallet: null,
      selectedAccount: '',
      selectedAccountAddress: '',
      stats: null,
    }
  },
  computed: {
    accountsAsArr() {
      return Array.from(this.wallet.get(WalletAccountsKey).entries())
    },
  },
  watch: {
    selectedAccount(name) {
      const account = this.wallet.get(WalletAccountsKey).get(name)
      this.selectedAccountAddress = account.address
      this.load()
    },
  },
  methods: {
    async load() {
      try {
        let res = await fetch(`/indexer/stats?address=${this.selectedAccountAddress}`, {
          method: 'GET',
        })
        switch (res.status) {
          case 200:
            break
          default:
            console.error(res)
            return
        }
        let data = await res.json()
        this.stats = data
      } catch (e) {
        console.error(e)
        return
      }
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
  <div class="local-row">
    <select v-model="selectedAccount">
      <option disabled value="" selected>Select an account</option>
      <option v-for="[key, value] in accountsAsArr" :key="key" :value="key">
        {{ value.name }}
      </option>
    </select>
  </div>
  <div class="card local-row" v-if="stats">
    <div class="card-header">Statistic</div>
    <div class="card-content">
      <div class="card-field">
        <span class="card-field-label">Landings</span>
        <span class="card-field-value">{{ stats.landings }}</span>
      </div>
      <div class="card-field">
        <span class="card-field-label">Amount (income or expense)</span>
        <span class="card-field-value">{{ stats.amount }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
select {
  margin-top: 25px;
}
</style>
