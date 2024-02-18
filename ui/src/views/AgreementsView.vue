<script>
import { WalletLocalStorageKey, WalletAccountsKey } from '@/constants/index.js'
import { ReadWallet } from '../utils'
import { ethers } from 'ethers'

export default {
  data() {
    return {
      wallet: null,
      selectedAccount: '',
      selectedAccountAddress: '',
      agreements: [],
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
      this.load(account.address)
    },
  },
  methods: {
    async load(address) {
      try {
        let url
        if (address) url = `/indexer/agreements?address=${address}`
        else url = `/indexer/agreements`
        let res = await fetch(url, { method: 'GET' })
        switch (res.status) {
          case 200:
            break
          default:
            console.error(res)
            return
        }
        let data = await res.json()
        for (let i = 0; i < data.length; i++) {
          data[i].amount = ethers.formatEther(ethers.parseEther(data[i].amount))
        }
        this.agreements = data
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
    this.load()
  },
}
</script>

<template>
  <div class="container">
    <div class="item">
      <select v-model="selectedAccount">
        <option disabled value="" selected>Select an account</option>
        <option v-for="[key, value] in accountsAsArr" :key="key" :value="key">
          {{ value.name }}
        </option>
      </select>
    </div>
  </div>
  <h1>Agreements</h1>
  <div>
    <table v-if="agreements.length">
      <thead>
        <tr>
          <th>Station</th>
          <th>Entity</th>
          <th>Price (ETH)</th>
          <th>Signed</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="{ station, entity, amount, signed } in agreements" :key="station">
          <td>{{ station }}</td>
          <td>{{ entity }}</td>
          <td>{{ amount }}</td>
          <td>{{ signed ? 'False' : 'True' }}</td>
        </tr>
      </tbody>
    </table>
    <p v-else>There are no agreements at the moment.</p>
  </div>
</template>

<style scoped>
button {
  width: 100%;
}

select {
  margin: 25px 0 0 0;
  width: 75%;
}
</style>
